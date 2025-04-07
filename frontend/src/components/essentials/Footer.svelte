<script lang="ts">
    import {
        Footer,
        FooterCopyright,
        FooterLinkGroup,
        FooterLink,
        FooterBrand
    } from "flowbite-svelte";
</script>

<Footer
    footerType="socialmedia"
    class="border-t border-gray-200 bg-gray-50 dark:border-gray-600 dark:bg-gray-800"
>
    <div class="container mx-auto px-4 sm:py-12 md:px-0">
        <div class="md:flex md:justify-between">
            <div class="grid grid-cols-2 gap-8 sm:grid-cols-6 sm:gap-6">
                <div class="col-span-2 mb-6 md:mb-0">
                    <FooterBrand
                        href="https://github.com/riseupgroup"
                        src="https://github.com/riseupgroup.png"
                        alt="RiseUpGroup Logo"
                        name="RiseUpGroup"
                        imgClass="me-3 h-8 w-8"
                    />
                    <p class="mt-4 max-w-lg text-gray-500 dark:text-gray-400">
                        We are a group of developers who are passionate about creating beautiful and
                        functional web applications.
                    </p>
                </div>
                <div>
                    <h2 class="mb-6 text-sm font-semibold uppercase text-gray-900 dark:text-white">
                        Resources & tools
                    </h2>
                    <FooterLinkGroup>
                        <FooterLink liClass="mb-4" href="https://actix.rs">Actix Web</FooterLink>
                        <FooterLink liClass="mb-4" href="https://www.sea-ql.org/SeaORM/"
                            >SeaORM</FooterLink
                        >
                        <FooterLink liClass="mb-4" href="https://flowbite-svelte.com"
                            >Flowbite Svelte</FooterLink
                        >
                        <FooterLink liClass="mb-4" href="https://kit.svelte.dev"
                            >Svelte Kit</FooterLink
                        >
                    </FooterLinkGroup>
                </div>
                <div>
                    <h2 class="mb-6 text-sm font-semibold uppercase text-gray-900 dark:text-white">
                        Our GitHubs
                    </h2>
                    <FooterLinkGroup>
                        <FooterLink liClass="mb-4" href="https://github.com/riseupgroup"
                            >RiseUpGroup</FooterLink
                        >
                        <FooterLink liClass="mb-4" href="https://github.com/comet1903"
                            >Comet1903</FooterLink
                        >
                        <FooterLink liClass="mb-4" href="https://github.com/untbu">Untbu</FooterLink
                        >
                    </FooterLinkGroup>
                </div>
                <div>
                    <h2 class="mb-6 text-sm font-semibold uppercase text-gray-900 dark:text-white">
                        Used in this project
                    </h2>
                    <FooterLinkGroup>
                        <FooterLink
                            liClass="mb-4"
                            href="https://github.com/riseupgroup/authentication-service"
                            >Authentication Service</FooterLink
                        >
                        <FooterLink liClass="mb-4" href="https://auth.riseupgroup.net"
                            >Auth Server</FooterLink
                        >
                        <FooterLink liClass="mb-4" href="https://go.riseupgroup.net/youtube-lib"
                            >YouTube Library</FooterLink
                        >
                    </FooterLinkGroup>
                </div>
            </div>
        </div>
        <hr class="my-6 border-gray-200 sm:mx-auto lg:my-8 dark:border-gray-700" />
        <div class="sm:flex sm:items-center sm:justify-center">
            <FooterCopyright
                href="https://github.com/riseupgroup"
                by="RiseUpGroup, Comet1903 & Untbu"
            />
        </div>
    </div>
</Footer>
