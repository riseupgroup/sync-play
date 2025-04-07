// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
    interface Window {
        user: User | null;
        userPromise: Promise<User | null> | null;
        refreshUser: () => Promise<User | null>;
        getUser: () => Promise<User | null>;
    }
}

export type User = {
    id: string;
    name: string;
};

export enum MouseButton {
    Left = 0,
    Middle = 1,
    Right = 2,
    Back = 3,
    Forward = 4
}

export class MouseClick {
    button: MouseButton;

    constructor(MouseEvent: MouseEvent) {
        switch (MouseEvent.button) {
            case 0:
                if (MouseEvent.ctrlKey || MouseEvent.metaKey) {
                    this.button = MouseButton.Middle;
                } else {
                    this.button = MouseButton.Left;
                }
                break;
            case 1:
                this.button = MouseButton.Middle;
                break;
            case 2:
                this.button = MouseButton.Right;
                break;
            default:
                this.button = MouseButton.Left;
                break;
        }
    }

    goto(url: string) {
        switch (this.button) {
            case MouseButton.Left:
                window.location.href = url;
                break;
            case MouseButton.Middle:
                window.open(url, "_blank");
                break;
            default:
                break;
        }
    }
}

export function getLocalTimestamp(d: Date): string {
    return (
        d.getFullYear() +
        "-" +
        (d.getMonth() + 1).toString().padStart(2, "0") +
        "-" +
        d.getDate().toString().padStart(2, "0") +
        "T" +
        d.getHours().toString().padStart(2, "0") +
        ":" +
        d.getMinutes().toString().padStart(2, "0") +
        ":" +
        d.getSeconds().toString().padStart(2, "0")
    );
}
