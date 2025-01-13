import * as net from 'net';
import { Worker, isMainThread, parentPort, workerData } from 'worker_threads';

const PORT_NUM: number = 1024;
const HOST_ADDR: string = "192.168.0.1";

const startTime: number = Date.now();

function scanPort(port: number): Promise<void> {
    return new Promise<void>((resolve) => {
        const socket: net.Socket = new net.Socket();
        socket.setTimeout(50);

        socket.connect(port, HOST_ADDR, () => {
            console.log(`port ${port} is open on ${HOST_ADDR}`);
            socket.destroy();
            resolve();
        });

        socket.on('error', (err: NodeJS.ErrnoException) => {
            if (err.code === 'ECONNREFUSED') {
            } else {
                console.log(`error on port ${port}: ${err.message}`);
            }
            socket.destroy();
            resolve();
        });

        socket.on('timeout', () => {
            socket.destroy();
            resolve();
        });
    });
}

async function threadNumHandler(maxThreads: number): Promise<void> {
    for (let i = 0; i < PORT_NUM + 1; i += maxThreads) {
        const workers: Promise<void>[] = Array.from({ length: maxThreads }, (_, j) => scanPort(i + j));
        await Promise.all(workers);
    }
}

async function main(): Promise<void> {
    await threadNumHandler(8); 
    const endTime: number = Date.now();
    console.log(`execution time: ${endTime - startTime} ms`);
}

main();


