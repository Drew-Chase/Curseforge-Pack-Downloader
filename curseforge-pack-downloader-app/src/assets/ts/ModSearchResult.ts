export type Pagination = {
    index: number;
    pageSize: number;
    resultCount: number;
    totalCount: number;
}

export type PackItem = {
    id: number;
    gameId: number;
    name: string;
    slug: string;
    links: {
        websiteUrl: string;
        wikiUrl: string;
        issuesUrl: string | null;
        sourceUrl: string | null;
    };
    summary: string;
    status: number;
    downloadCount: number;
    isFeatured: boolean;
    primaryCategoryId: number;
    classId: number;
    categories: Category[];
    authors: Author[];
    logo: Logo;
    mainFileId: number;
    screenshots: Screenshot[];
    latestFilesIndexes: LatestFileIndex[];
    dateCreated: string;
    dateModified: string;
    dateReleased: string;
    allowModDistribution: boolean;
    gamePopularityRank: number;
    isAvailable: boolean;
    thumbsUpCount: number;
}

export type Category = {
    id: number;
    gameId: number;
    name: string;
    slug: string;
    url: string;
    iconUrl: string;
    dateModified: string;
    isClass: boolean;
    classId: number;
    parentCategoryId: number;
    latestFile: LatestFile[];
}
export type Logo = {
    id: number;
    modId: number;
    title: string;
    description: string;
    thumbnailUrl: string;
    url: string;
}


export type LatestFile = {
    id: number;
    gameId: number;
    modId: number;
    isAvailable: boolean;
    displayName: string;
    fileName: string;
    releaseType: number;
    fileStatus: number;
    hashes: Array<{
        value: string;
        algo: number;
    }>;
    fileDate: string;
    fileLength: number;
    downloadCount: number;
    downloadUrl: string;
    gameVersions: string[];
    sortableGameVersions: Array<{
        gameVersionName: string;
        gameVersionPadded: string;
        gameVersion: string;
        gameVersionReleaseDate: string;
        gameVersionTypeId: number;
    }>;
    dependencies: any[];
    alternateFileId: number;
    isServerPack: boolean;
    fileFingerprint: number;
    modules: Array<{
        name: string;
        fingerprint: number;
    }>;
};

export type Screenshot = {
    id: number;
    modId: number;
    title: string;
    description: string;
    thumbnailUrl: string;
    url: string;
}
export type Author = {
    id: number;
    name: string;
    url: string;
    avatarUrl: string;
}


export type LatestFileIndex = {
    gameVersion: string;
    fileId: number;
    filename: string;
    releaseType: number;
    gameVersionTypeId: number;
    modLoader: number;
}

export type ModSearchResult = {
    data: PackItem[];
    pagination: Pagination;
}