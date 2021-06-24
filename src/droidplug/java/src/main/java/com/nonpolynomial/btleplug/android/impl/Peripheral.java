package com.nonpolynomial.btleplug.android.impl;

import android.bluetooth.BluetoothAdapter;
import android.bluetooth.BluetoothDevice;
import android.bluetooth.BluetoothGatt;
import android.bluetooth.BluetoothGattCallback;

import java.util.LinkedList;
import java.util.Queue;

import gedgygedgy.rust.future.Future;

@SuppressWarnings("unused") // Native code uses this class.
class Peripheral {
    private final BluetoothDevice device;
    private BluetoothGatt gatt;
    private final Callback callback;
    private boolean connected = false;

    private final Queue<Future.Waker<Void>> connectQueue = new LinkedList<>();
    private final Queue<Future.Waker<Void>> disconnectQueue = new LinkedList<>();

    public Peripheral(String address) {
        this.device = BluetoothAdapter.getDefaultAdapter().getRemoteDevice(address);
        this.callback = new Callback();
    }

    public Future<Void> connect() {
        return asyncWithWaker((waker) -> {
            synchronized (this) {
                if (this.gatt == null) {
                    try {
                        this.connectQueue.add(waker);
                        this.gatt = this.device.connectGatt(null, false, this.callback);
                    } catch (SecurityException ex) {
                        throw new PermissionDeniedException(ex);
                    }
                } else if (this.connected) {
                    waker.wake(null);
                } else if (this.gatt.connect()) {
                    this.connectQueue.add(waker);
                } else {
                    throw new RuntimeException("Unable to reconnect to device");
                }
            }
        });
    }

    public Future<Void> disconnect() {
        return asyncWithWaker((waker) -> {
            synchronized (this) {
                if (this.gatt == null || !this.connected) {
                    waker.wake(null);
                } else {
                    this.gatt.disconnect();
                    this.disconnectQueue.add(waker);
                }
            }
        });
    }

    public boolean isConnected() {
        return this.connected;
    }

    private class Callback extends BluetoothGattCallback {
        @Override
        public void onConnectionStateChange(BluetoothGatt gatt, int status, int newState) {
            synchronized (Peripheral.this) {
                if (status == BluetoothGatt.GATT_SUCCESS) {
                    switch (newState) {
                        case BluetoothGatt.STATE_CONNECTED:
                            Peripheral.this.connected = true;
                            wakeQueue(Peripheral.this.connectQueue, null);
                            break;
                        case BluetoothGatt.STATE_DISCONNECTED:
                            Peripheral.this.connected = false;
                            wakeQueue(Peripheral.this.disconnectQueue, null);
                            break;
                    }
                } else if (newState == BluetoothGatt.STATE_DISCONNECTED) {
                    wakeQueueWithThrowable(Peripheral.this.connectQueue, new NotConnectedException());
                }
            }
        }
    }

    private static <T> void wakeQueue(Queue<Future.Waker<T>> queue, T result) {
        while (!queue.isEmpty()) {
            Future.Waker<T> waker = queue.remove();
            waker.wake(result);
        }
    }

    private static <T> void wakeQueueWithThrowable(Queue<Future.Waker<T>> queue, Throwable result) {
        while (!queue.isEmpty()) {
            Future.Waker<T> waker = queue.remove();
            waker.wakeWithThrowable(result);
        }
    }

    private interface AsyncWithWaker<T> {
        void withWaker(Future.Waker<T> waker);
    }

    private static <T> Future<T> asyncWithWaker(AsyncWithWaker<T> callback) {
        Future.Waker<T> waker = Future.create();
        try {
            callback.withWaker(waker);
        } catch (Throwable ex) {
            waker.wakeWithThrowable(ex);
        }
        return waker.getFuture();
    }
}
