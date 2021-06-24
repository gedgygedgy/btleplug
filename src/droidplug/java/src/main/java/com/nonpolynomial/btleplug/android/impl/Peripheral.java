package com.nonpolynomial.btleplug.android.impl;

import android.bluetooth.BluetoothAdapter;
import android.bluetooth.BluetoothDevice;
import android.bluetooth.BluetoothGatt;
import android.bluetooth.BluetoothGattCallback;
import android.bluetooth.BluetoothGattCharacteristic;

import java.util.LinkedList;
import java.util.Queue;

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
                                if (status == BluetoothGatt.GATT_SUCCESS) {
                                    if (newState == BluetoothGatt.STATE_CONNECTED) {
                                        Peripheral.this.wakeCommand(waker, null);
                                    }
                                } else {
                                    throw new NotConnectedException();
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
                                    if (status == BluetoothGatt.GATT_SUCCESS) {
                                        if (newState == BluetoothGatt.STATE_DISCONNECTED) {
                                            Peripheral.this.wakeCommand(waker, null);
                                        }
                                    } else {
                                        throw new RuntimeException("Unable to disconnect");
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
    }
}
