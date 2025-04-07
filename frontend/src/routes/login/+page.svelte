<script lang="ts">
    import { A, Avatar, Card, Button } from "flowbite-svelte";
    import { LinkOutline } from "flowbite-svelte-icons";
    import { page } from "$app/stores";

    function login() {
        let path = $page.url.searchParams.get("path");
        if (path != null) {
            document.cookie = "path=" + path + "; path=/";
        }
        window.location.href = "/auth";
    }
</script>

<div class="flex flex-col items-center justify-center pt-4 md:h-screen">
    <Card class="w-full">
        <a
            href="/"
            class="mb-4 flex items-center justify-center text-2xl font-semibold text-black lg:mb-6 dark:text-white"
        >
            <Avatar
                src="https://github.com/riseupgroup.png"
                rounded
                size="lg"
                class="mr-4"
                alt="Logo"
            ></Avatar>
            <div class="text-left">
                <div>Actix-Svelte Example</div>
                <div class="text-sm font-medium text-gray-600 dark:text-gray-400">
                    a <A href="https://github.com/riseupgroup">RiseUpGroup</A> project
                </div>
            </div>
        </a>
        <div class="space-y-6">
            <div class="mb-4 text-sm font-medium text-gray-500 dark:text-white">
                <Button class="w-full" on:click={login}
                    >Login using Auth Server<LinkOutline /></Button
                >
            </div>
            <div class="text-sm font-medium text-gray-500 dark:text-gray-400">
                <A
                    on:click={() => {
                        window.open("https://auth.riseupgroup.net");
                    }}>Auth Server</A
                > is a secure user authentication service that can manage users and teams
            </div>
            <hr />
            <div class="text-xs font-medium text-gray-400 dark:text-gray-300">
                A <A href="https://github.com/riseupgroup">RiseUpGroup 🚀</A> project by <A
                    href="https://github.com/comet1903">Comet1903</A
                > & <A href="https://github.com/untbu">Untbu</A>
            </div>
        </div>
    </Card>
</div>
