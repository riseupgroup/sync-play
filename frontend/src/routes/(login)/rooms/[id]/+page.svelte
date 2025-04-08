<script lang="ts">
    import { page } from "$app/stores";
    import { onMount } from "svelte";
    import { PlayerCommands } from "../../../../app";

    let fileUrl: string | null = null;
    let fileInput: HTMLInputElement;
    let video: HTMLVideoElement;
    let socket: WebSocket | null = null;

    let blockEventListenerVal: boolean = false;
    let blockEventListenerTimeout: number | null = null;
    let isControlling = false;

    let timeDifferenceLog = 0;

    // due to a bug in safari, we need to check if the browser is safari -- https://bugs.webkit.org/show_bug.cgi?id=163433
    // @ts-ignore
    let isSafari = /constructor/i.test(window.HTMLElement) || (function (p) { return p.toString() === "[object SafariRemoteNotification]"; })(!window['safari'] || (typeof safari !== 'undefined' && window['safari'].pushNotification));

    onMount(() => {
        fileInput.addEventListener(
            "change",
            () => {
                if (fileInput.files == null) {
                    return;
                }
                let file = fileInput.files[0];
                fileUrl = URL.createObjectURL(file);
                video.load();
                handleVideoPlayer();
            },
            false
        );

        let wsUrl = "://" + window.location.host + "/api/rooms/" + $page.params.id + "/ws";
        wsUrl = window.location.protocol == "https:" ? "wss" + wsUrl : "ws" + wsUrl;

        try {
            socket = new WebSocket(wsUrl);
            let interval = setInterval(() => send(PlayerCommands.Ping, ""), 1000);
            handleSocket();
            return () => {
                socket?.close();
                clearInterval(interval);
            };
        } catch (err) {
            alert(err);
        }
    });

    function handleSocket() {
        if (socket != null) {
            socket.onmessage = (e) => {
                let eventData: string = e.data;
                let semicolonIndex = eventData.indexOf(";");
                let command = parseInt(eventData.substring(0, semicolonIndex));
                let data = eventData.substring(semicolonIndex + 1);

                if (command == PlayerCommands.Play) {
                    blockEventListenerFn();
                    video.currentTime = parseFloat(data);
                    video.play();
                    isControlling = false;
                } else if (command == PlayerCommands.SetTime) {
                    blockEventListenerFn();
                    video.currentTime = parseFloat(data);
                } else if (command == PlayerCommands.UpdateTime) {
                    let otherTime = parseFloat(data);
                    let timeDifference = video.currentTime - otherTime;
                    let tmp = timeDifference;
                    timeDifference = (timeDifference + timeDifferenceLog) / 2;
                    timeDifferenceLog = tmp;
                    let time_difference_abs = Math.abs(timeDifference);

                    if (video.paused) video.play();

                    if (isSafari) {
                        if (time_difference_abs > 2) {
                            video.currentTime = otherTime + 0.5;
                        }
                        return;
                    }

                    if (time_difference_abs > 3) {
                        video.currentTime = otherTime;
                    } else if (time_difference_abs > 0.2) {
                        let speed = Math.min(time_difference_abs * 0.2, 0.1);
                        speed = 1 + (timeDifference > 0 ? -speed : speed);
                        video.playbackRate = speed;
                    } else {
                        if (video.playbackRate != 1.0) {
                            if (time_difference_abs < 0.1) {
                                video.playbackRate = 1.0;
                            }
                        }
                    }
                } else if (command == PlayerCommands.Pause) {
                    blockEventListenerFn();
                    video.pause();
                    isControlling = false;
                } else {
                    console.info(data);
                }
            };
        }
    }

    function handleVideoPlayer() {
        video.addEventListener("play", () => {
            if (blockEventListenerVal) return;
            send(PlayerCommands.Play, (video.currentTime + 0.1).toString());
            isControlling = true;
        });
        video.addEventListener("pause", () => {
            if (blockEventListenerVal) return;
            send(PlayerCommands.Pause, "");
            isControlling = false;
        });
        video.addEventListener("timeupdate", () => {
            if (video.paused && !blockEventListenerVal)
                send(PlayerCommands.SetTime, video.currentTime.toString());
            else if (isControlling)
                send(PlayerCommands.UpdateTime, (video.currentTime + 0.1).toString());
        });
    }

    function send(command: PlayerCommands, data: string) {
        if (socket != null) {
            if (socket.readyState == WebSocket.OPEN) {
                socket.send(command + ";" + data);
            } else {
                socket = null;
                alert("Socket not open");
            }
        }
    }

    function blockEventListenerFn() {
        if (blockEventListenerTimeout) clearTimeout(blockEventListenerTimeout);
        blockEventListenerVal = true;
        blockEventListenerTimeout = setTimeout(() => (blockEventListenerVal = false), 5);
    }
</script>

{#if fileUrl == null}
    <div id="input-container">
        <input type="file" bind:this={fileInput} accept="video/*" />
    </div>
{/if}

<!-- svelte-ignore a11y-media-has-caption -->
<video bind:this={video} controls style={fileUrl == null ? "display: none;" : ""}>
    <source src={fileUrl} />
</video>

<style>
    video {
        max-height: 60vh;
        max-width: 100%;
        margin-left: 50%;
        transform: translateX(-50%);
    }
</style>
