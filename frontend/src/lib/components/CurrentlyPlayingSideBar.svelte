<script lang="ts">
    import {onMount} from 'svelte';
    import {listen} from '@tauri-apps/api/event'
    import {invoke} from '@tauri-apps/api/core';
    import type {Song} from "$lib/model";

    export let image_url: string;

    let current_song: string = "not playing anything";
    let artist: string = "No Artist"

    onMount(() => {
        invoke("do_emit", {})
        const unlisten = listen('current-song', async (event) => {
            let new_song: string = event.payload.title;
            const song = {
                title: new_song,
                artist: event.payload.artist
            } satisfies  Song
            artist = song.artist
            if (current_song !== new_song) {
                current_song = event.payload.title;
                let temp_url: string = await invoke('generate_image', {"song": song});
                let x = temp_url.replace(/^["']|["']$/g, '')
                console.log(x)
                image_url = x;
            }
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

<aside class="bg-tertiary-500">
    <div class="flex items-center mb-4">
        <img src={current_song} alt="Artist" class=""/>
    </div>
    <div class="text-center mb-4">
        <p class="text-white">{current_song}</p>
        <p class="font-semibold text-white">{artist}</p>
        <p class="text-gray-200">Ultraviolence</p>
    </div>
    <div class="text-center">
        <div class="btn-group variant-filled">
            <button on:click={previousSong}>
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="size-5">
                    <path d="M7.712 4.818A1.5 1.5 0 0 1 10 6.095v2.972c.104-.13.234-.248.389-.343l6.323-3.906A1.5 1.5 0 0 1 19 6.095v7.81a1.5 1.5 0 0 1-2.288 1.276l-6.323-3.905a1.505 1.505 0 0 1-.389-.344v2.973a1.5 1.5 0 0 1-2.288 1.276l-6.323-3.905a1.5 1.5 0 0 1 0-2.552l6.323-3.906Z"/>
                </svg>
            </button>
            <button on:click={playPause}>
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="size-5">
                    <path d="M12.75 4a.75.75 0 0 0-.75.75v10.5c0 .414.336.75.75.75h.5a.75.75 0 0 0 .75-.75V4.75a.75.75 0 0 0-.75-.75h-.5ZM17.75 4a.75.75 0 0 0-.75.75v10.5c0 .414.336.75.75.75h.5a.75.75 0 0 0 .75-.75V4.75a.75.75 0 0 0-.75-.75h-.5ZM3.288 4.819A1.5 1.5 0 0 0 1 6.095v7.81a1.5 1.5 0 0 0 2.288 1.277l6.323-3.906a1.5 1.5 0 0 0 0-2.552L3.288 4.819Z"/>
                </svg>
            </button>
            <button on:click={nextSong}>
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="size-5">
                    <path d="M3.288 4.818A1.5 1.5 0 0 0 1 6.095v7.81a1.5 1.5 0 0 0 2.288 1.276l6.323-3.905c.155-.096.285-.213.389-.344v2.973a1.5 1.5 0 0 0 2.288 1.276l6.323-3.905a1.5 1.5 0 0 0 0-2.552l-6.323-3.906A1.5 1.5 0 0 0 10 6.095v2.972a1.506 1.506 0 0 0-.389-.343L3.288 4.818Z"/>
                </svg>
            </button>
        </div>
    </div>
</aside>