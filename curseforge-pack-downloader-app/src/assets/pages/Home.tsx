import {Button, Progress, Spinner} from "@nextui-org/react";
import PackItemComponent from "../components/PackItemComponent.tsx";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faArchive} from "@fortawesome/free-solid-svg-icons";
import OTooltip from "../components/Extends/OTooltip.tsx";
import OInput from "../components/Extends/OInput.tsx";
import {useEffect, useState} from "react";
import {ProcessProgressResponse, search_modpacks, unpack_modpack, unpack_modpack_file} from "../ts/curseforge.ts";
import {ModSearchResult} from "../ts/ModSearchResult.ts";
import {useAlertModal} from "../providers/AlertModalProvider.tsx";
import {open} from "@tauri-apps/plugin-shell";
import {open as openFile} from "@tauri-apps/plugin-dialog";

export default function Home()
{
    const [search, setSearch] = useState("");
    const [items, setItems] = useState<ModSearchResult | null>(null);
    const [unpackProgress, setUnpackProgress] = useState<ProcessProgressResponse | null>(null);
    const {alert} = useAlertModal();

    useEffect(() =>
    {
        search_modpacks(search).then(setItems);
    }, [search]);

    return (
        <>
            {unpackProgress &&
                <div className={"fixed top-0 left-0 w-full h-full bg-black/50 z-50 flex justify-center items-center backdrop-blur-sm flex-col"}>
                    <div className={"flex justify-center items-center w-1/2 mx-auto flex-col"}>

                        <Spinner
                            color={"primary"}
                            label={unpackProgress.message || "Unpacking..."}
                            size={"lg"}
                        />
                        <Progress
                            className={"w-1/2 mt-4"}
                            minValue={0}
                            maxValue={1}
                            value={unpackProgress.progress}
                            color={"primary"}
                            size={"sm"}
                        />
                    </div>
                </div>
            }
            <div className={"flex flex-col justify-center gap-4"}>
                <div className={"flex flex-col justify-center items-center mx-4 gap-4"}>
                    <div className={"flex flex-row gap-4 w-full items-center"}>
                        <OInput
                            placeholder={"All the mods 10, 925201, etc..."}
                            label={"Search"}
                            autoComplete={"off"}
                            value={search}
                            onValueChange={setSearch}
                        />
                        <OTooltip content={`Select pack archive from the file system.`}>
                            <Button className={"min-w-0 h-12"} onClick={async () =>
                            {
                                const file = await openFile({
                                    title: "Select a pack archive",
                                    directory: false,
                                    multiple: false,
                                    canCreateDirectories: false,
                                    filters: [
                                        {name: "Modpack Archive", extensions: ["zip"]}
                                    ]
                                });

                                if (!file) return;

                                const path = await openFile({
                                    title: "Select a location to save the pack",
                                    directory: true,
                                    multiple: false,
                                    canCreateDirectories: true
                                });

                                if (!path) return;

                                try
                                {
                                    await unpack_modpack_file(file, path, setUnpackProgress);
                                } catch
                                {
                                    console.error("Failed to unpack modpack.");
                                    alert({
                                        title: "Failed to unpack",
                                        type: "error",
                                        message: "Failed to unpack modpack"
                                    });
                                }
                                setUnpackProgress(null);
                                alert({
                                    title: "Unpack Finished",
                                    type: "info",
                                    message: "Modpack unpacked successfully.",
                                    actions: [
                                        {
                                            label: "Open",
                                            onClick: () =>
                                            {
                                                open(path);
                                            }
                                        }
                                    ]
                                });


                            }}>
                                <FontAwesomeIcon icon={faArchive}/>
                            </Button>
                        </OTooltip>
                    </div>
                    <div
                        className={"w-full dark:bg-[#242424] bg-neutral-50 max-h-[calc(100dvh_-_160px)] overflow-y-auto rounded-lg shadow-lg gap-2 flex flex-col p-2"}
                    >
                        {items?.data.map((item, i) => (
                            <PackItemComponent
                                key={i}
                                id={item.id}
                                name={item.name}
                                description={item.summary}
                                author={item.authors[0].name}
                                iconUrl={item.logo.thumbnailUrl}
                                gameVersion={item.latestFilesIndexes[0].gameVersion}
                                websiteUrl={item.links.websiteUrl}
                                onUnpack={async path =>
                                {
                                    try
                                    {
                                        await unpack_modpack(item.id, path, setUnpackProgress);
                                    } catch
                                    {
                                        console.error("Failed to unpack modpack.");
                                        alert({
                                            title: "Failed to unpack",
                                            type: "error",
                                            message: "Failed to unpack modpack"
                                        });
                                    }
                                    setUnpackProgress(null);
                                    alert({
                                        title: "Unpack Finished",
                                        type: "info",
                                        message: "Modpack unpacked successfully.",
                                        actions: [
                                            {
                                                label: "Open",
                                                onClick: () =>
                                                {
                                                    open(path);
                                                }
                                            }
                                        ]
                                    });
                                }}
                            />
                        ))}
                    </div>
                </div>
            </div>
        </>
    );
}
