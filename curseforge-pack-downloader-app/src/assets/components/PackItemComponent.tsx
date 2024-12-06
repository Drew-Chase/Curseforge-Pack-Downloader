import {Avatar, Button, Chip, Link} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faDownload} from "@fortawesome/free-solid-svg-icons";
import {open} from "@tauri-apps/plugin-shell";
import {open as openFile} from "@tauri-apps/plugin-dialog";

interface PackItemProps
{
    id: number;
    name: string;
    description: string;
    author: string;
    iconUrl: string;
    gameVersion: string;
    websiteUrl: string;
    onUnpack: (path:string) => void;
}


export default function PackItemComponent(props: PackItemProps)
{
    return (
        <div className={"flex flex-col shrink-0 p-4 justify-between dark:bg-[#101010] bg-black/5 hover:dark:bg-black/70 hover:bg-black/10 rounded-lg transition-colors"}>
            <div className={"flex flex-row shrink-0"}>
                <Avatar src={props.iconUrl} radius={"sm"} size={"lg"} className={"mr-4"}/>
                <div className={"flex flex-col items-start"}>
                    <div className={"flex flex-row items-center"}>
                        <p className={"text-large font-bold mr-2"}>{props.name}</p>
                        <p className={"text-tiny opacity-70"}>By {props.author}</p>
                    </div>
                    <p className={"opacity-70"}>{props.description}</p>
                </div>
            </div>
            <div className={"flex flex-row mt-4 gap-3 items-center"}>
                <div className={"flex flex-row gap-3 mr-auto"}>
                    <Chip radius={"sm"} color={"default"} variant={"flat"}>
                        <span className={"flex flex-row gap-2 items-center"}>
                            {props.gameVersion}
                        </span>
                    </Chip>
                </div>
                <Button
                    as={Link}
                    variant={"flat"}
                    showAnchorIcon
                    isExternal
                    onClick={
                        async () =>
                        {
                            await open(props.websiteUrl);
                        }
                    }
                >
                    View on Curseforge
                </Button>
                <Button
                    color={"primary"}
                    startContent={<FontAwesomeIcon icon={faDownload} className={"mr-1"}/>}
                    onClick={async () =>
                    {
                        const path = await openFile({
                            title: "Select a location to save the pack",
                            defaultPath: `${props.name}-${props.author}-${props.id}-${new Date().getTime()}`,
                            directory: true,
                            multiple: false,
                            canCreateDirectories: true
                        });
                        if (path)
                        {
                            props.onUnpack(path);
                        }
                    }}
                >
                    Download
                </Button>
            </div>
        </div>
    );
}