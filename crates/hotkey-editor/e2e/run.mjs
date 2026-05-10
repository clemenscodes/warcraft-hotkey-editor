import { createConnection } from "net";
import { spawn } from "child_process";

function checkPort(port) {
    return new Promise((resolve) => {
        const socket = createConnection({ port, host: "localhost", timeout: 1000 });
        socket.on("connect", () => { socket.destroy(); resolve(true); });
        socket.on("error", () => resolve(false));
        socket.on("timeout", () => { socket.destroy(); resolve(false); });
    });
}

function startServer() {
    return new Promise((resolve, reject) => {
        const proc = spawn(
            "dx",
            ["serve", "--package", "hotkey-editor", "--platform", "web"],
            { stdio: ["ignore", "pipe", "pipe"] }
        );

        process.on("exit", () => proc.kill());

        const timeout = setTimeout(
            () => reject(new Error("App did not compile within 120s")),
            120_000
        );

        const onData = (chunk) => {
            process.stdout.write(chunk);
            if (chunk.toString().includes("launching app")) {
                clearTimeout(timeout);
                proc.stdout.off("data", onData);
                proc.stderr.off("data", onData);
                proc.stdout.on("data", (c) => process.stdout.write(c));
                proc.stderr.on("data", (c) => process.stderr.write(c));
                resolve();
            }
        };

        proc.stdout.on("data", onData);
        proc.stderr.on("data", onData);
    });
}

if (await checkPort(8080)) {
    console.log("Dev server already running, reusing.");
} else {
    process.stdout.write("Waiting for app to compile...\n");
    await startServer();
}

const tests = spawn("pnpm", ["exec", "playwright", "test", ...process.argv.slice(2)], {
    stdio: "inherit",
});

tests.on("exit", (code) => process.exit(code ?? 0));
