/**
 * nx rust executor
 */


import { ExecutorContext } from "@nx/devkit";
import { exec } from "child_process";

export interface RustExecutorOptions {
    command: string;
}

export default async function rust_executor(
    options: RustExecutorOptions,
    context: ExecutorContext,
) {
    return new Promise((resolve) => {
        exec(
            `cargo ${options.command}`, 
            { cwd: context.cwd }, 
            (error, stdout, stderr) => {
                console.log(stdout);
                console.error(stderr);
                resolve({ success: !error})
            },
        );     
    });
};
