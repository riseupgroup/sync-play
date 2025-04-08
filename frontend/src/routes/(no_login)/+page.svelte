<script lang="ts">
    import { Button } from "flowbite-svelte";
    import {
        ArrowRightOutline,
        LockSolid,
        ShareNodesSolid,
        InboxFullSolid
    } from "flowbite-svelte-icons";
    import { onMount } from "svelte";

    onMount(async () => {
        await window.getUser();
    });

    function getStarted() {
        window.location.href = window.user == null ? "/login" : "/rooms";
    }
</script>

<section>
    <div class="mb-12">
        <div class="mr-auto place-self-center lg:col-span-7">
            <h1
                class="mb-4 max-w-2xl text-4xl font-extrabold leading-none tracking-tight text-gray-900 md:text-5xl xl:text-6xl dark:text-gray-50"
            >
                Watch together<br />but different
            </h1>
            <p class="mb-6 max-w-2xl font-light md:text-lg lg:mb-8 lg:text-xl">
                Our sync tool lets you and your friends watch the same movie at the exact same time
                – no hosting, no streaming. Everyone uses their own file. We just keep things in
                sync.
            </p>
            <Button on:click={getStarted}>
                Get started<ArrowRightOutline />
            </Button>
        </div>
    </div>
</section>

<section>
    <div class="mb-8 max-w-screen-lg">
        <h2 class="mb-4 text-4xl font-extrabold tracking-tight text-gray-900 dark:text-gray-50">
            How It Works 🎬
        </h2>
        <p class="sm:text-lg">
            Each user loads the same video file from their local system. Once you’re in a room,
            our WebSocket-based sync logic takes over: play, pause, seek – everything stays
            aligned. No video or audio ever leaves your computer.
        </p>
    </div>
    <div class="space-y-8 md:grid md:grid-cols-2 md:gap-12 md:space-y-0 lg:grid-cols-3">
        <div>
            <div
                class="mb-4 flex h-10 w-10 items-center justify-center rounded-full bg-primary-100 lg:h-12 lg:w-12 dark:bg-primary-900"
            >
                <ShareNodesSolid />
            </div>
            <h3 class="mb-2 text-xl font-bold text-gray-900 dark:text-gray-50">
                Sync & Catch-up
            </h3>
            <p>
                Whether it’s pausing, seeking, or adjusting speed – our smart sync logic keeps
                all participants perfectly in time, with auto-correction and smooth catch-up
                mechanisms.
            </p>
        </div>
        <div>
            <div
                class="mb-4 flex h-10 w-10 items-center justify-center rounded-full bg-primary-100 lg:h-12 lg:w-12 dark:bg-primary-900"
            >
                <InboxFullSolid />
            </div>
            <h3 class="mb-2 text-xl font-bold text-gray-900 dark:text-gray-50">Room-Based</h3>
            <p>
                Create or join a room with your friends. Every room has its own sync session.
                Only people in the same room control and affect the playback.
            </p>
        </div>
        <div>
            <div
                class="mb-4 flex h-10 w-10 items-center justify-center rounded-full bg-primary-100 lg:h-12 lg:w-12 dark:bg-primary-900"
            >
                <LockSolid />
            </div>
            <h3 class="mb-2 text-xl font-bold text-gray-900 dark:text-gray-50">Privacy</h3>
            <p>
                No files are uploaded, no streams are shared. All content stays local, and we
                only exchange simple sync signals over secure WebSockets. Everyone watcher needs <span
                    class="font-bold">their own copy</span
                > of the video.
            </p>
        </div>
    </div>
    <div class="max-w-screen-lg py-8 sm:text-lg lg:py-16">
        <h2 class="mb-4 text-4xl font-extrabold tracking-tight text-gray-900 dark:text-gray-50">
            Why we built this 🛠️
        </h2>
        <p class="mb-4 sm:text-xl">
            Not everyone has the bandwidth – literally – to host and stream movies from home.
            Maybe your upload speed is too slow, or setting up a media server like Jellyfin ends
            in endless buffering and out-of-sync frustration.
        </p>
        <p class="sm:text-xl">
            We built this tool because we just wanted to hit play and watch together – without
            the technical drama. No server setup. No transcoding delays. No rebuffering after
            every pause. Everyone uses their own copy of the movie, locally. Our tool just keeps
            your players in sync, using lightweight WebSocket communication. Simple. Fast.
            Reliable. You don’t need to stream when you already have the file.
        </p>
    </div>
    <div class="max-w-screen-lg sm:text-lg">
        <h2 class="mb-4 text-4xl font-extrabold tracking-tight text-gray-900 dark:text-gray-50">
            About us, RiseUpGroup 🚀
        </h2>
        <p class="sm:text-xl">
            At RiseUpGroup, we focus on empowering individuals through technology and
            innovation. Our SyncPlay is just one of the ways we simplify digital life, creating
            tools that add real value and enhance everyday experiences.
        </p>
    </div>

</section>
