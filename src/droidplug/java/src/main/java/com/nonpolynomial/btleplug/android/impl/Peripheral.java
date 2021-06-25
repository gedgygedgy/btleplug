package com.nonpolynomial.btleplug.android.impl;

import android.bluetooth.BluetoothAdapter;
import android.bluetooth.BluetoothDevice;
import android.bluetooth.BluetoothGatt;
import android.bluetooth.BluetoothGattCallback;
import android.bluetooth.BluetoothGattCharacteristic;
import android.bluetooth.BluetoothGattService;

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;
import java.util.Queue;
import java.util.UUID;

import gedgygedgy.rust.future.Future;

@SuppressWarnings("unused") // Native code uses this class.
class Peripheral {
    private final BluetoothDevice device;
    private BluetoothGatt gatt;
    private final Callback callback;
    private boolean connected = false;

    private final Queue<Runnable> commandQueue = new LinkedList<>();
    private boolean executingCommand = false;
    private CommandCallback commandCallback;

    public Peripheral(String address) {
        this.device = BluetoothAdapter.getDefaultAdapter().getRemoteDevice(address);
        this.callback = new Callback();
    }

    public Future<Void> connect() {
        Future.Waker<Void> waker = Future.create();
        synchronized (this) {
            this.queueCommand(() -> {
                this.asyncWithWaker(waker, () -> {
                    CommandCallback callback = new CommandCallback() {
                        @Override
                        public void onConnectionStateChange(BluetoothGatt gatt, int status, int newState) {
                            Peripheral.this.asyncWithWaker(waker, () -> {
                                if (status != BluetoothGatt.GATT_SUCCESS) {
                                    throw new NotConnectedException();
                                }

                                if (newState == BluetoothGatt.STATE_CONNECTED) {
                                    Peripheral.this.wakeCommand(waker, null);
                                }
                            });
                        }
                    };

                    if (this.connected) {
                        Peripheral.this.wakeCommand(waker, null);
                    } else if (this.gatt == null) {
                        try {
                            this.setCommandCallback(callback);
                            this.gatt = this.device.connectGatt(null, false, this.callback);
                        } catch (SecurityException ex) {
                            throw new PermissionDeniedException(ex);
                        }
                    } else {
                        this.setCommandCallback(callback);
                        if (!this.gatt.connect()) {
                            throw new RuntimeException("Unable to reconnect to device");
                        }
                    }
                });
            });
        }
        return waker.getFuture();
    }

    public Future<Void> disconnect() {
        Future.Waker<Void> waker = Future.create();
        synchronized (this) {
            this.queueCommand(() -> {
                this.asyncWithWaker(waker, () -> {
                    if (!this.connected) {
                        Peripheral.this.wakeCommand(waker, null);
                    } else {
                        this.setCommandCallback(new CommandCallback() {
                            @Override
                            public void onConnectionStateChange(BluetoothGatt gatt, int status, int newState) {
                                Peripheral.this.asyncWithWaker(waker, () -> {
                                    if (status != BluetoothGatt.GATT_SUCCESS) {
                                        throw new RuntimeException("Unable to disconnect");
                                    }

                                    if (newState == BluetoothGatt.STATE_DISCONNECTED) {
                                        Peripheral.this.wakeCommand(waker, null);
                                    }
                                });
                            }
                        });
                        this.gatt.disconnect();
                    }
                });
            });
        }
        return waker.getFuture();
    }

    public boolean isConnected() {
        return this.connected;
    }

    public Future<byte[]> read(UUID uuid) {
        Future.Waker<byte[]> waker = Future.create();
        synchronized (this) {
            this.queueCommand(() -> {
                this.asyncWithWaker(waker, () -> {
                    if (!this.connected) {
                        throw new NotConnectedException();
                    }

                    BluetoothGattCharacteristic characteristic = this.getCharacteristicByUuid(uuid);
                    this.setCommandCallback(new CommandCallback() {
                        @Override
                        public void onCharacteristicRead(BluetoothGatt gatt, BluetoothGattCharacteristic characteristic, int status) {
                            Peripheral.this.asyncWithWaker(waker, () -> {
                                if (!characteristic.getUuid().equals(uuid)) {
                                    throw new UnexpectedCharacteristicException();
                                }

                                Peripheral.this.wakeCommand(waker, characteristic.getValue());
                            });
                        }
                    });
                    if (!this.gatt.readCharacteristic(characteristic)) {
                        throw new RuntimeException("Unable to read characteristic");
                    }
                });
            });
        }
        return waker.getFuture();
    }

    public Future<Void> write(UUID uuid, byte[] data, int writeType) {
        Future.Waker<Void> waker = Future.create();
        synchronized (this) {
            this.queueCommand(() -> {
                this.asyncWithWaker(waker, () -> {
                    if (!this.connected) {
                        throw new NotConnectedException();
                    }

                    BluetoothGattCharacteristic characteristic = this.getCharacteristicByUuid(uuid);
                    characteristic.setValue(data);
                    characteristic.setWriteType(writeType);
                    this.setCommandCallback(new CommandCallback() {
                        @Override
                        public void onCharacteristicWrite(BluetoothGatt gatt, BluetoothGattCharacteristic characteristic, int status) {
                            Peripheral.this.asyncWithWaker(waker, () -> {
                                if (!characteristic.getUuid().equals(uuid)) {
                                    throw new UnexpectedCharacteristicException();
                                }

                                Peripheral.this.wakeCommand(waker, null);
                            });
                        }
                    });
                    if (!this.gatt.writeCharacteristic(characteristic)) {
                        throw new RuntimeException("Unable to read characteristic");
                    }
                });
            });
        }
        return waker.getFuture();
    }

    public Future<List<BluetoothGattCharacteristic>> discoverCharacteristics() {
        Future.Waker<List<BluetoothGattCharacteristic>> waker = Future.create();
        synchronized (this) {
            this.queueCommand(() -> {
                this.asyncWithWaker(waker, () -> {
                    if (!this.connected) {
                        throw new NotConnectedException();
                    }

                    this.setCommandCallback(new CommandCallback() {
                        @Override
                        public void onServicesDiscovered(BluetoothGatt gatt, int status) {
                            if (status != BluetoothGatt.GATT_SUCCESS) {
                                throw new RuntimeException("Unable to discover services");
                            }

                            Peripheral.this.wakeCommand(waker, Peripheral.this.getCharacteristics());
                        }
                    });
                    if (!this.gatt.discoverServices()) {
                        throw new RuntimeException("Unable to discover services");
                    }
                });
            });
        }
        return waker.getFuture();
    }

    private List<BluetoothGattCharacteristic> getCharacteristics() {
        List<BluetoothGattCharacteristic> result = new ArrayList<>();
        if (this.gatt != null) {
            for (BluetoothGattService service : this.gatt.getServices()) {
                result.addAll(service.getCharacteristics());
            }
        }
        return result;
    }

    private BluetoothGattCharacteristic getCharacteristicByUuid(UUID uuid) {
        for (BluetoothGattCharacteristic characteristic : this.getCharacteristics()) {
            if (characteristic.getUuid().equals(uuid)) {
                return characteristic;
            }
        }

        throw new NoSuchCharacteristicException();
    }

    private void queueCommand(Runnable callback) {
        if (this.executingCommand) {
            this.commandQueue.add(callback);
        } else {
            this.executingCommand = true;
            callback.run();
        }
    }

    private void setCommandCallback(CommandCallback callback) {
        assert this.commandCallback == null;
        this.commandCallback = callback;
    }

    private void runNextCommand() {
        assert this.executingCommand;
        this.commandCallback = null;
        if (this.commandQueue.isEmpty()) {
            this.executingCommand = false;
        } else {
            Runnable callback = this.commandQueue.remove();
            callback.run();
        }
    }

    private <T> void wakeCommand(Future.Waker<T> waker, T result) {
        waker.wake(result);
        this.runNextCommand();
    }

    private <T> void asyncWithWaker(Future.Waker<T> waker, Runnable callback) {
        try {
            callback.run();
        } catch (Throwable ex) {
            waker.wakeWithThrowable(ex);
            this.runNextCommand();
        }
    }

    private class Callback extends BluetoothGattCallback {
        @Override
        public void onConnectionStateChange(BluetoothGatt gatt, int status, int newState) {
            synchronized (Peripheral.this) {
                switch (newState) {
                    case BluetoothGatt.STATE_CONNECTED:
                        Peripheral.this.connected = true;
                        break;
                    case BluetoothGatt.STATE_DISCONNECTED:
                        Peripheral.this.connected = false;
                        break;
                }
                if (Peripheral.this.commandCallback != null) {
                    Peripheral.this.commandCallback.onConnectionStateChange(gatt, status, newState);
                }
            }
        }

        @Override
        public void onCharacteristicRead(BluetoothGatt gatt, BluetoothGattCharacteristic characteristic, int status) {
            synchronized (Peripheral.this) {
                if (Peripheral.this.commandCallback != null) {
                    Peripheral.this.commandCallback.onCharacteristicRead(gatt, characteristic, status);
                }
            }
        }

        @Override
        public void onCharacteristicWrite(BluetoothGatt gatt, BluetoothGattCharacteristic characteristic, int status) {
            synchronized (Peripheral.this) {
                if (Peripheral.this.commandCallback != null) {
                    Peripheral.this.commandCallback.onCharacteristicWrite(gatt, characteristic, status);
                }
            }
        }

        @Override
        public void onServicesDiscovered(BluetoothGatt gatt, int status) {
            synchronized (Peripheral.this) {
                if (Peripheral.this.commandCallback != null) {
                    Peripheral.this.commandCallback.onServicesDiscovered(gatt, status);
                }
            }
        }
    }

    private static abstract class CommandCallback extends BluetoothGattCallback {
        @Override
        public void onConnectionStateChange(BluetoothGatt gatt, int status, int newState) {
            throw new UnexpectedCallbackException();
        }

        @Override
        public void onCharacteristicRead(BluetoothGatt gatt, BluetoothGattCharacteristic characteristic, int status) {
            throw new UnexpectedCallbackException();
        }

        @Override
        public void onCharacteristicWrite(BluetoothGatt gatt, BluetoothGattCharacteristic characteristic, int status) {
            throw new UnexpectedCallbackException();
        }

        @Override
        public void onServicesDiscovered(BluetoothGatt gatt, int status) {
            throw new UnexpectedCallbackException();
        }
    }
}
