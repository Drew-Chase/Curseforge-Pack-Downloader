import {cn, Input, InputProps} from "@nextui-org/react";

export default function OInput(props: InputProps)
{
    return (
        <Input
            classNames={
                {
                    ...props.classNames, inputWrapper:
                        cn(
                            "dark:bg-[#242424] bg-neutral-50",
                            "dark:data-[focus]:!bg-neutral-800 dark:data-[hover]:!bg-neutral-800",
                            "data-[focus]:!bg-neutral-100 data-[hover]:!bg-neutral-100",
                            props.classNames?.inputWrapper
                        )
                }
            }
            {...props}
        />
    );
}