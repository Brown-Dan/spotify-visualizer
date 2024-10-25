<script lang="ts">
    import {invoke} from '@tauri-apps/api/core';
    import type {Song} from "$lib/model";

    let artist = '';
    let title = '';
    let image_url: string;

    async function handleSubmit(event: Event) {
        event.preventDefault();
        const song = {
            title: title,
            artist: artist
        } satisfies  Song

        image_url = await invoke('generate_image', {"song": song});
        image_url = image_url.replace(/^["']|["']$/g, '')
    }
</script>

<form on:submit={handleSubmit}>
    <input type="text" name="title">
    <input type="text" name="artist">
    <button type="submit">Submit</button>
</form>

{#if image_url}
    <img src="{image_url}" alt="generated">
{/if}

