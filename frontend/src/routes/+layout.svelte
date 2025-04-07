<script lang="ts">
    import "../app.css";
    import { page } from "$app/stores";
    import { goto } from "$app/navigation";
    import { SveltePathFinder } from "svelte-path-finder";

    let show: boolean = false;

    let finder: SveltePathFinder | null;
    async function getFinder(): Promise<SveltePathFinder> {
        if (finder != null) return finder;
        let res = await fetch("/svelte_path_finder.json");
        if (res.ok) {
            finder = new SveltePathFinder().fromJson(await res.json());
            return finder;
        } else {
            alert(await res.text());
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            return null as any;
        }
    }

    page.subscribe(async (page) => {
        let path = trimPath(page.url.pathname);
        let requireLogin = (await getFinder()).find(path);

        if (requireLogin) {
            let user = await window.getUser();
            if (user == null) {
                show = false;
                goto("/login?path=/" + path, { replaceState: true });
                return;
            }
        }
        show = true;
    });

    function trimPath(path: string): string {
        let start = path[0] == "/" ? 1 : 0;
        let end = path.length;
        if (path[end - 1] == "/" && end - 1 >= start) {
            end -= 1;
        }
        if (start != 0 || end != path.length) {
            return path.substring(start, end);
        }
        return path;
    }
</script>

{#if show}
    <slot />
{/if}
