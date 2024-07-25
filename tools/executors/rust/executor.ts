import { ExecutorContext } from "@nx/devkit";
import { exec } from "child_process";
import * as path from "path";

export interface RustExecutorOptions {
    command: string;
    projectRoot: string;
    args?: string | string[];
}

export default async function rust_executor(
    options: RustExecutorOptions,
    context: ExecutorContext,
) {
    const projectRoot = path.join(context.root, options.projectRoot);
    
    // Handle args whether it's a string or an array of strings
    let args = '';
    if (typeof options.args === 'string') {
        args = options.args;
    } else if (Array.isArray(options.args)) {
        args = options.args.join(' ');
    }
    
    // Construct the command, including any additional arguments
    const command = `cargo ${options.command}${args ? ' -- ' + args : ''}`;
    
    return new Promise((resolve) => {
        console.log(`Executing command: ${command}`);
        exec(
            command, 
            { cwd: projectRoot },
            (error, stdout, stderr) => {
                console.log(stdout);
                console.error(stderr);
                resolve({ success: !error})
            },
        );     
    });
}