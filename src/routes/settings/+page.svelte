<script>
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { _ } from 'svelte-i18n';
	import { locale } from 'svelte-i18n';

	let theme = "light";
	let notifications = true;
	let apiKey = "";
	let apiService = "thegamesdb";
	let language = "en";
	let showToast = false;
	let toastMsg = '';
	let toastType = 'success';

	onMount(async () => {
		try {
			apiKey = await invoke('get_setting', { key: 'thegamesdb_api_key' });
			apiService = await invoke('get_setting', { key: 'api_service' }) || 'thegamesdb';
			language = await invoke('get_setting', { key: 'language' }) || 'en';
			$locale = language;
		} catch (e) {
			console.error('Errore nel caricamento delle impostazioni:', e);
		}
	});

	async function saveSettings() {
		try {
			await invoke('set_setting', { key: 'thegamesdb_api_key', value: apiKey });
			await invoke('set_setting', { key: 'api_service', value: apiService });
			await invoke('set_setting', { key: 'language', value: language });
			$locale = language;
			toastMsg = $_('settings.settings_saved');
			toastType = 'success';
			showToast = true;
			setTimeout(() => { showToast = false; }, 2500);
		} catch (e) {
			toastMsg = $_('settings.error_saving') + e;
			toastType = 'error';
			showToast = true;
			setTimeout(() => { showToast = false; }, 2500);
		}
	}

	async function openAPIWebsite() {
		const url = apiService === 'thegamesdb' ? 'https://thegamesdb.net/' : 'https://rawg.io/apidocs';
		try {
			await invoke('open_url', { url });
		} catch (e) {
			toastMsg = $_('settings.error_opening_link') + e;
			toastType = 'error';
			showToast = true;
			setTimeout(() => { showToast = false; }, 2500);
		}
	}
</script>

<div class="relative min-h-screen flex flex-col" style="min-height: 100vh;">
	<div class="absolute inset-0 w-full h-full" style="background: linear-gradient(135deg, #f0f0f0 0%, #c9ffe7 70%, #e0c9ff 100%); opacity: 0.7; backdrop-filter: blur(12px); z-index: 0;"></div>
	<header class="w-full pt-5 px-5">
		<div class="w-full bg-white/50 backdrop-blur-sm rounded-lg border border-black/50 p-4 sm:p-2 text-center shadow-lg mx-auto">
			<div class="mb-1">
				<h1 class="text-2xl font-semibold text-gray-900 mb-1">{$_('settings.title')}</h1>
				<p class="text-gray-700">
					{$_('settings.description')}
				</p>
			</div>
		</div>
	</header>
	<main class="flex-grow flex justify-center items-center px-4">
		<div class="max-w-lg w-full bg-white/90 backdrop-blur-sm rounded-lg border border-white/20 p-8 shadow-lg">
			<form class="space-y-6">
				<div class="text-left">
					<label class="block font-medium mb-1">{$_('settings.api_service')}</label>
					<select bind:value={apiService} class="border rounded px-3 py-2 w-full">
						<option value="thegamesdb">{$_('settings.thegamesdb')}</option>
						<option value="rawg">{$_('settings.rawg')}</option>
					</select>
				</div>
				<div class="text-left">
					<label class="block font-medium mb-1">{$_('settings.api_key')} {apiService === 'thegamesdb' ? $_('settings.thegamesdb') : $_('settings.rawg')}</label>
					<input type="password" bind:value={apiKey} placeholder="Inserisci la tua chiave API" class="border rounded px-3 py-2 w-full" />
					<p class="text-sm text-gray-600 mt-1">
						{$_('settings.get_key')}
						<button class="text-blue-500 underline" on:click={openAPIWebsite}>{apiService === 'thegamesdb' ? 'thegamesdb.net' : 'rawg.io'}</button>
					</p>
				</div>
				<div class="text-left">
					<label class="block font-medium mb-1">{$_('settings.language')}</label>
					<select bind:value={language} class="border rounded px-3 py-2 w-full">
						<option value="en">{$_('settings.english')}</option>
						<option value="it">{$_('settings.italian')}</option>
					</select>
				</div>
				<div class="mt-6 text-center">
					<button type="button" on:click={saveSettings} class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded flex items-center gap-2">
						<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7H5a2 2 0 00-2 2v9a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-3m-1 4l-3 3m0 0l-3-3m3 3V4"></path>
						</svg>
						{$_('settings.save')}
					</button>
				</div>
			</form>
		</div>
	</main>
</div>
{#if showToast}
	<div class="fixed bottom-4 right-4 z-50 text-white px-4 py-2 rounded shadow-lg" class:bg-green-500={toastType === 'success'} class:bg-red-500={toastType === 'error'}>
		{toastMsg}
	</div>
{/if}
