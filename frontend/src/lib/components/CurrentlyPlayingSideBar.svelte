<script lang="ts">
    import {onMount} from 'svelte';
    import {listen} from '@tauri-apps/api/event'
    import {invoke} from '@tauri-apps/api/core';
    import {Avatar} from "@skeletonlabs/skeleton";

    export let image_url: string;

    let current_song: string = ""
    let current_song_image: string = ""
    let artist: string = ""
    let album: string = ""

    onMount(() => {
        invoke("do_emit", {})
        const unlisten = listen('current-song', async (event) => {
            current_song = event.payload.title;
            artist = event.payload.artist.name;
            image_url = event.payload.image_url;
            current_song_image = event.payload.artist[0].image_url
            album = event.payload.album_name;
        });
        return () => {
            unlisten.then((dispose) => dispose())
        }
    });

    async function previousSong() {
        await invoke('previous_song', {})
    }

    async function playPause() {
        await invoke('pause_play', {})
    }

    async function nextSong() {
        await invoke('next_song', {})
    }
</script>

<aside class="bg-surface-500 p-6 shadow-lg">
    <div class="flex justify-center mb-4">
        <Avatar src={current_song_image} width="w-32" rounded="rounded-full" />
    </div>
    <div class="text-center mb-4">
        <p class="text-white text-lg font-semibold">{current_song}</p>
        <p class="text-white text-sm font-medium">{artist}</p>
        <p class="text-gray-200 text-xs">{album}</p>
    </div>
    <div class="flex justify-center">
        <div class="btn-group variant-filled">
            <button
                    on:click={previousSong}>
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="size-5 w-5 h-5 mr-1">
                    <path d="M7.712 4.818A1.5 1.5 0 0 1 10 6.095v2.972c.104-.13.234-.248.389-.343l6.323-3.906A1.5 1.5 0 0 1 19 6.095v7.81a1.5 1.5 0 0 1-2.288 1.276l-6.323-3.905a1.505 1.505 0 0 1-.389-.344v2.973a1.5 1.5 0 0 1-2.288 1.276l-6.323-3.905a1.5 1.5 0 0 1 0-2.552l6.323-3.906Z"/>
                </svg>
            </button>
            <button
                    on:click={playPause}
            >
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="size-5 w-5 h-5 mr-1">
                    <path d="M12.75 4a.75.75 0 0 0-.75.75v10.5c0 .414.336.75.75.75h.5a.75.75 0 0 0 .75-.75V4.75a.75.75 0 0 0-.75-.75h-.5ZM17.75 4a.75.75 0 0 0-.75.75v10.5c0 .414.336.75.75.75h.5a.75.75 0 0 0 .75-.75V4.75a.75.75 0 0 0-.75-.75h-.5ZM3.288 4.819A1.5 1.5 0 0 0 1 6.095v7.81a1.5 1.5 0 0 0 2.288 1.277l6.323-3.906a1.5 1.5 0 0 0 0-2.552L3.288 4.819Z"/>
                </svg>
            </button>
            <button
                    on:click={nextSong}
            >
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="size-5 w-5 h-5 mr-1">
                    <path d="M3.288 4.818A1.5 1.5 0 0 0 1 6.095v7.81a1.5 1.5 0 0 0 2.288 1.276l6.323-3.905c.155-.096.285-.213.389-.344v2.973a1.5 1.5 0 0 0 2.288 1.276l6.323-3.905a1.5 1.5 0 0 0 0-2.552l-6.323-3.906A1.5 1.5 0 0 0 10 6.095v2.972a1.506 1.506 0 0 0-.389-.343L3.288 4.818Z"/>
                </svg>
            </button>
        </div>
    </div>
</aside>
