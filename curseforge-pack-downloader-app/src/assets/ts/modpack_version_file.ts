import {invoke} from "@tauri-apps/api/core";

type SortableGameVersion = {
    gameVersionName: string;
    gameVersionPadded: string;
    gameVersion: string;
    gameVersionReleaseDate: string;
    gameVersionTypeId: number;
};

type FileHash = {
    value: string;
    algo: number;
};

export type ModpackVersionFile = {
    id: number;
    gameId: number;
    modId: number;
    isAvailable: boolean;
    displayName: string;
    fileName: string;
    releaseType: number;
    fileStatus: number;
    hashes: FileHash[];
    fileDate: string;
    fileLength: number;
    downloadCount: number;
    fileSizeOnDisk: number;
    downloadUrl: string | null;
    gameVersions: string[];
    sortableGameVersions: SortableGameVersion[];
    alternateFileId: number;
    isServerPack: boolean;
    serverPackFileId: number;
};


export default async function get_pack_versions(id:number) : Promise<ModpackVersionFile[]>
{
	return invoke("get_pack_versions", {id: id})
}
