import {Channel, invoke} from "@tauri-apps/api/core";
import {ModSearchResult} from "./ModSearchResult.ts";

export interface ProcessProgressResponse
{
    stage: ProcessStage;
    progress: number;
    message: string;
}

enum ProcessStage
{
    ExtractingArchive,
    DownloadingArchive,
    DownloadingMods,
    Finalizing,
}


export async function search_modpacks(query: string): Promise<ModSearchResult>
{
    try
    {
        let response = await invoke("search_modpacks", {query: query}) as string;
        return JSON.parse(response) as ModSearchResult;
    } catch (err)
    {
        console.error("failed to search packs", err);
    }

    return {} as ModSearchResult;
}

export async function unpack_modpack(id: number, path: string, callback: (progress: ProcessProgressResponse) => void): Promise<void>
{
    const downloadEvent = new Channel<ProcessProgressResponse>();
    downloadEvent.onmessage = callback;
    await invoke("unpack", {id: id, output: path, onEvent: downloadEvent});
}
export async function unpack_modpack_file(file: string, path: string, callback: (progress: ProcessProgressResponse) => void): Promise<void>
{
    const downloadEvent = new Channel<ProcessProgressResponse>();
    downloadEvent.onmessage = callback;
    await invoke("unpack_file", {file: file, output: path, onEvent: downloadEvent});
}