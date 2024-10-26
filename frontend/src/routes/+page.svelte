<script lang="ts">
    import CurrentlyPlayingSideBar from "$lib/components/CurrentlyPlayingSideBar.svelte";
    import { ProgressRadial } from '@skeletonlabs/skeleton';


    let image_url: string;

</script>


<div class="h-[100vh] overflow-hidden grid grid-cols-[70%_30%]">
    <main class="space-y-4 bg-primary-500 p-4">
        <div class="p-4 overflow-auto">
            {#if image_url}
                <img src="{image_url}" alt="generated" class="mx-auto my-4 max-w-full rounded-md" />
            {:else}
                <div class="flex justify-center items-center h-screen">
                    <ProgressRadial />
                </div>
            {/if}
        </div>
    </main>
    <CurrentlyPlayingSideBar bind:image_url />
</div>
