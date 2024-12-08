import {useEffect, useState} from "react";
import {Button, Modal, ModalBody, ModalContent, ModalFooter, ModalHeader, Pagination, Table, TableBody, TableCell, TableColumn, TableHeader, TableRow} from "@nextui-org/react";
import get_pack_versions, {ModpackVersionFile} from "../ts/modpack_version_file.ts";
import Conversions from "../ts/conversions.ts";

interface PackVersionSelectModalProps
{
    onClose: () => void;
    onConfirm: (file_id: number) => void;
    packId: number | null;
}

export default function PackVersionSelectModal(props: PackVersionSelectModalProps)
{
    const [packId, setPackId] = useState<number | null>(props.packId);
    const [fileItems, setFileItems] = useState<ModpackVersionFile[]>([]);
    const [pageItems, setPageItems] = useState<ModpackVersionFile[]>([]);
    const [page, setPage] = useState(1);
    const itemsPerPage = 20;


    useEffect(() =>
    {
        setPackId(props.packId);
        if (props.packId != null)
        {
            get_pack_versions(props.packId).then(setFileItems);
        }
    }, [props.packId]);

    useEffect(() =>
    {
        if (fileItems.length == 0) return;
        setPageItems(fileItems.slice((page - 1) * itemsPerPage, page * itemsPerPage));
    }, [page, fileItems]);


    return (
        <Modal isOpen={packId != null} onClose={props.onClose} size="5xl">
            <ModalContent>
                {onClose => (
                    <>
                        <ModalHeader>
                            Select a pack version
                        </ModalHeader>
                        <ModalBody>

                            <Table
                                removeWrapper
                                className={"max-h-[calc(100dvh_-_300px)] overflow-auto"}
                                isHeaderSticky
                                isStriped

                            >
                                <TableHeader>
                                    <TableColumn>Version</TableColumn>
                                    <TableColumn>Released</TableColumn>
                                    <TableColumn className={"w-0 min-w-0"} hideHeader>Actions</TableColumn>
                                </TableHeader>
                                <TableBody>
                                    {Array.from(pageItems).map((item, index) =>
                                    {

                                        return (
                                            <TableRow key={`${props.packId}-files-${index}`} className={"hover:bg-black/10"}>
                                                <TableCell>{item.displayName}</TableCell>
                                                <TableCell>{Conversions.formatTimeClosestRelative(new Date(item.fileDate))}</TableCell>
                                                <TableCell>
                                                    <Button
                                                        color={"primary"}
                                                        onClick={() =>
                                                        {
                                                            props.onConfirm(item.id);
                                                            onClose();
                                                        }}
                                                    >
                                                        Download
                                                    </Button>
                                                </TableCell>
                                            </TableRow>
                                        );
                                    })}
                                </TableBody>
                            </Table>

                        </ModalBody>
                        <ModalFooter>
                            <div className={"flex flex-row justify-center w-full max-w-[calc(100%_-_90px)]"}>
                                <Pagination
                                    variant={"flat"}
                                    showShadow
                                    total={Math.ceil(fileItems.length / itemsPerPage)}
                                    page={page}
                                    onChange={setPage}
                                    hidden={itemsPerPage > fileItems.length}
                                />
                            </div>
                            <Button onClick={onClose}>Close</Button>
                        </ModalFooter>;
                    </>
                )
                }
            </ModalContent>
        </Modal>
    );
}