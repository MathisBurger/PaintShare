import {useMediaQuery} from "react-responsive";
import Sidebar from "./sidebar";

import style from "../styles/base.module.css";

// This function is used to make the
// web UI more responsive. It handles how
// much columns should be shown at a
// specific screen size
export const useScreenType = () => {

    const is3Cols = useMediaQuery({minWidth: 1440});
    const is2Cols = useMediaQuery({minWidth: 1265});
    const is1Cols = useMediaQuery({minWidth: 800});

    if (is3Cols) {
        return "3-cols";
    }
    if (is2Cols) {
        return "2-cols";
    }
    if (is1Cols) {
        return "1-cols";
    }
    return "fullscreen";
}

// This react component handles the responsive
// design with conditional rendering.
// It has static rules, what should be shown at
// what screen-size
export default function DesignWrapper(props: any) {

    let screenType = useScreenType();

    let screen = null;

    switch (screenType) {
        case "3-cols":
            screen = (
                <div className={style.baseLayout}>
                    <Sidebar />
                    {props.children}
                </div>
            );
            break;
        case "2-cols":
            screen = (
                <div className={style.baseLayout}>
                    <Sidebar />
                    {props.children}
                </div>
            );
            break;
        case "1-cols":
            screen = (
                <div className={style.baseLayout}>
                    <Sidebar />
                    {props.children}
                </div>
            );
            break;
        default:
            screen = (
                <>
                    {props.children}
                </>
            );
            break;
    }
    return screen;
}
