import {Navbar, NavbarContent, NavbarItem} from "@nextui-org/react";
import ThemeSwitcher from "./ThemeSwitcher.tsx";
import CurseforgeLogo from "../images/curseforge_logo.svg.tsx";

export default function Navigation()
{

    return (
        <Navbar maxWidth={"full"}>
            <NavbarContent justify={"start"}>
                <CurseforgeLogo width={250} height={80}/>
            </NavbarContent>

            <NavbarContent justify="end">
                <NavbarItem>
                    <ThemeSwitcher/>
                </NavbarItem>
            </NavbarContent>
        </Navbar>);
}