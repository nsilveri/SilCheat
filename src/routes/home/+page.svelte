<script>
import { onMount } from 'svelte';
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import { _ } from 'svelte-i18n';

  let imageUrl = "";
  const defaultImage = "https://placehold.co/160x90?text=No+Image";
  let cardWidth = 180; // px
  let cardHeight = 170; // px
  let imageHeight = 90; // px

  let showModal = false;
  let fileNames = [];
  let fileError = '';
  let loading = false;
  let showImageModal = false;
  let selectedTable = null;
  let showToast = false;
  let toastMsg = '';
  let toastType = 'success';
  let isTauri = false;
  let tables = [];
  let tableToDelete = null;
  let showDeleteModal = false;
  let apiService = 'thegamesdb';

  onMount(async () => {
    isTauri = typeof window !== 'undefined' && !!window.__TAURI__;
    await loadTables();
    try {
      apiService = await invoke('get_setting', { key: 'api_service' }) || 'thegamesdb';
    } catch (e) {
      console.error('Errore nel caricamento del servizio API:', e);
    }
  });

  async function loadTables() {
    try {
      tables = await invoke('get_tables');
    } catch (e) {
      toastMsg = $_('home.error_loading') + e;
      toastType = 'error';
      showToast = true;
      setTimeout(() => { showToast = false; }, 2500);
    }
  }

  function chiediElimina(table) {
    tableToDelete = table;
    showDeleteModal = true;
  }
  function openImageModal(table) {
    selectedTable = table;
    showImageModal = true;
  }
  async function confermaElimina() {
    if (tableToDelete !== null) {
      try {
                await invoke('delete_table', { tableName: tableToDelete });
        toastMsg = $_('home.table_deleted', { values: { table: tableToDelete } });
        toastType = 'success';
        await loadTables();
      } catch (e) {
        toastMsg = $_('home.error_deleting', { values: { table: tableToDelete } }) + e;
        toastType = 'error';
      }
      showToast = true;
      setTimeout(() => { showToast = false; }, 2000);
      showDeleteModal = false;
      tableToDelete = null;
    }
  }
  async function selectAndSetImage() {
    if (!selectedTable) return;
    try {
      const selected = await open({
        multiple: false,
        filters: [{ name: 'Images', extensions: ['png', 'jpg', 'jpeg', 'gif', 'bmp'] }]
      });
      if (selected) {
        const imagePath = Array.isArray(selected) ? selected[0] : selected;
        await invoke('set_table_image', { tableName: selectedTable.name, imagePath: imagePath });
        toastMsg = $_('home.image_set');
        toastType = 'success';
        await loadTables();
      }
    } catch (e) {
      toastMsg = $_('home.error_setting_image') + e;
      toastType = 'error';
    }
    showToast = true;
    setTimeout(() => { showToast = false; }, 2500);
    showImageModal = false;
    selectedTable = null;
  }

  // Fetch logo from remote scraper and set it for the selected table
  async function fetchAndSetLogo() {
    if (!selectedTable) return;
    try {
      // Use selected table name as the game name to search
      const res = await invoke('fetch_and_set_logo', { tableName: selectedTable.name, gameName: selectedTable.name });
      toastMsg = typeof res === 'string' ? res : $_('home.image_set');
      toastType = 'success';
      // refresh tables so image appears
      await loadTables();
    } catch (e) {
      toastMsg = $_('home.error_setting_image') + (e ? ' ' + e : '');
      toastType = 'error';
      console.error('fetchAndSetLogo error:', e);
    }
    showToast = true;
    setTimeout(() => { showToast = false; }, 2500);
    showImageModal = false;
    selectedTable = null;
  }

  async function deleteImage() {
    if (!selectedTable) return;
    try {
      await invoke('delete_table_image', { tableName: selectedTable.name });
      toastMsg = $_('home.image_deleted');
      toastType = 'success';
      await loadTables();
    } catch (e) {
      toastMsg = $_('home.error_deleting_image') + e;
      toastType = 'error';
    }
    showToast = true;
    setTimeout(() => { showToast = false; }, 2500);
    showImageModal = false;
    selectedTable = null;
  }
  let isDragOver = false;

  function openModal() {
    showModal = true;
    fileNames = [];
    fileError = '';
  }

  async function selectAndImportFiles() {
    if (isTauri) {
      toastMsg = `Apertura dialogo file...`;
      toastType = 'success';
      showToast = true;
      const selected = await open({
        multiple: true,
        filters: [{ name: 'CHT', extensions: ['cht'] }]
      });
      toastMsg = `Selected ${selected}`;
      toastType = 'success';
      showToast = true;
      setTimeout(() => { showToast = false; }, 2500);
      if (selected) {
        const files = Array.isArray(selected) ? selected : [selected];
        fileNames = files.map(f => f.split(/[\\/]/).pop());
        fileError = '';
        loading = true;
        toastMsg = 'Importazione in corso...';
        toastType = 'error';
        showToast = true;
        setTimeout(() => { showToast = false; }, 2500);
        for (const filePath of files) {
          try {
            const res = await invoke('import_cht', { filePath: filePath });
            toastMsg = res;
            toastType = 'success';
            // Refresh the home tables list so the newly imported table appears
            await loadTables();
          } catch (err) {
            toastMsg = typeof err === 'string' ? err : $_('home.import_error_generic');
            toastType = 'error';
          }
          showToast = true;
          setTimeout(() => { showToast = false; }, 2500);
        }
        loading = false;
      }
    } else {
      // Fallback: usa input HTML (solo per interfaccia, non importa)
      toastMsg = $_('home.web_mode_error');
      toastType = 'error';
      showToast = true;
      setTimeout(() => { showToast = false; }, 2500);
    }
  }

  // Funzione per input HTML (solo interfaccia dimostrativa)
  function triggerFileInput() {
    const input = document.getElementById('file-input');
    if (input) input.click();
  }

  // Gestisce la selezione da input HTML (solo per demo)
  function handleHtmlFileSelect(event) {
    const files = Array.from(event.target.files);
      fileNames = files.map(f => f.name);
      fileError = 'Nota: il dialog HTML non fornisce percorsi completi. Usa Tauri per importare realmente.';
      toastMsg = $_('home.files_selected');
      toastType = 'error';
      showToast = true;
      setTimeout(() => { showToast = false; }, 3000);
  }
  function closeModal() {
    showModal = false;
    fileNames = [];
    fileError = '';
  }
  function handleFileChange(event) {
    const files = Array.from(event.target.files);
    processFiles(files);
  }

  function handleDrop(event) {
    event.preventDefault();
    isDragOver = false;
    const files = Array.from(event.dataTransfer.files);
    processFiles(files);
  }

  function handleDragOver(event) {
    event.preventDefault();
    isDragOver = true;
  }

  function handleDragLeave(event) {
    event.preventDefault();
    isDragOver = false;
  }

  async function processFiles(files) {
    if (!files.length) return;
    const invalid = files.filter(f => !f.name.endsWith('.cht'));
    if (invalid.length) {
      fileError = $_('home.invalid_files');
      fileNames = [];
      return;
    }
    fileNames = files.map(f => f.name);
    fileError = '';
    // Carica ogni file .cht nel backend Tauri
    loading = true;
    for (const file of files) {
      // Ottieni il percorso assoluto del file (solo se selezionato da dialog, non drag&drop da fuori app)
      // In Tauri, file.path è disponibile solo se usi l'API open dialog, altrimenti serve workaround
      // Qui usiamo file.path se disponibile, altrimenti file.name
      let filePath = file.path || file.name;
      // Se file.path non esiste, non puoi importare realmente il file
      // In produzione, usa l'API Tauri open dialog per ottenere il path
      // Per test, puoi passare file.name se il file è nella directory del progetto
      // eslint-disable-next-line no-undef
      const { invoke } = window.__TAURI__ ? window.__TAURI__.tauri : {};
      if (invoke && filePath) {
        try {
          const res = await invoke('import_cht', { filePath: filePath });
          toastMsg = res;
          toastType = 'success';
            // Refresh the home tables list after successful import
            await loadTables();
        } catch (err) {
          toastMsg = typeof err === 'string' ? err : $_('home.import_error');
          toastType = 'error';
        }
        showToast = true;
        setTimeout(() => { showToast = false; }, 2500);
      } else {
        toastMsg = $_('home.path_error');
        toastType = 'error';
        showToast = true;
        setTimeout(() => { showToast = false; }, 2500);
      }
    }
    loading = false;
  }
</script>

<div class="min-h-screen flex flex-col" style="background: linear-gradient(135deg, #c9ffe7 0%, #e9e9ff 70%, #dcecff 100%);">
  
  <!-- TOAST NOTIFICATIONS -->
  {#if showToast}
    <div class="fixed bottom-8 right-8 z-50 px-6 py-3 rounded shadow-lg animate-fadein font-semibold text-white"
      style="background-color: {toastType === 'success' ? '#22c55e' : '#ef4444'};">
      {toastMsg}
    </div>
  {/if}

  <!-- HEADER CONTENT -->
  <header class="w-full pt-5 px-5 fixed top-0 left-0 right-0 z-10 bg-transparent">
    <div class="w-full bg-white/50 backdrop-blur-sm rounded-lg border border-black/50 p-4 sm:p-2 shadow-lg flex items-center justify-between">
      <div></div>
      <div class="flex-1 text-center">
        <h1 class="text-2xl font-semibold text-gray-900 mb-1">{$_('home.title')}</h1>
        <p class="text-gray-700">{$_('home.description')}</p>
      </div>
      <div class="flex items-center gap-2 justify-end">
        <button class="bg-green-500 hover:bg-green-600 text-white font-bold py-2 px-4 rounded flex items-center gap-2" on:click={() => { showModal = true; }}>
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
          </svg>
          {$_('home.add')}
        </button>
      </div>
    </div>
  </header>

  <!-- IMPORT FILE MODAL -->
  {#if showModal}
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/40">
      <div
        class="bg-white rounded-lg shadow-lg p-6 w-full max-w-sm relative"
        role="region"
        aria-label="File upload area"
        on:drop={handleDrop}
        on:dragover={handleDragOver}
        on:dragleave={handleDragLeave}
        style="outline: {isDragOver ? '2px solid #22c55e' : 'none'}; outline-offset: 0;"
      >
        <h2 class="text-lg font-semibold mb-4">{$_('home.upload_title')}</h2>
        <div class="mb-4 flex flex-col items-center justify-center">
          <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded mb-2" on:click={selectAndImportFiles} disabled={loading}>
            {$_('home.select_files')}
          </button>
          {#if !isTauri}
            <button class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded mb-2" on:click={triggerFileInput}>
              {$_('home.choose_files')}
            </button>
            <input
              id="file-input"
              type="file"
              multiple
              accept=".cht"
              class="hidden"
              on:change={handleHtmlFileSelect}
            />
          {/if}
          {#if loading}
            <div class="mt-2 text-blue-600 font-semibold">{$_('home.loading')}</div>
          {/if}
          <div class="mt-2 text-sm text-gray-500">
            {#if isTauri}
              {$_('home.tauri_note')}
            {:else}
              {$_('home.web_note')}
            {/if}
          </div>
        </div>
        {#if fileNames.length}
          <div class="text-green-600 mb-2">
            {$_('home.selected_files')}
            <ul class="list-disc ml-4">
              {#each fileNames as name}
                <li>{name}</li>
              {/each}
            </ul>
          </div>
        {/if}
        {#if fileError}
          <div class="text-red-600 mb-2">{fileError}</div>
        {/if}
        <div class="flex justify-end gap-3 mt-4">
          <button class="bg-gray-300 hover:bg-gray-400 text-gray-800 font-bold py-2 px-4 rounded flex items-center gap-2" on:click={closeModal}>
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
            </svg>
            {$_('home.close')}
          </button>
        </div>
        {#if isDragOver}
          <div class="absolute inset-0 bg-green-200/40 pointer-events-none rounded-lg flex items-center justify-center text-green-700 text-lg font-bold">
            {$_('home.drop_here')}
          </div>
        {/if}
      </div>
    </div>
  {/if}

  <!-- MAIN CONTENT -->
  <main class="flex-grow pt-5 px-5 mb-8" style=" margin-top: 6rem; margin-bottom: 2rem;">
    <div class="w-full h-full overflow-y-auto grid gap-4 pb-20"
      style="grid-template-columns: repeat(auto-fit, minmax({cardWidth}px, 1fr)); scrollbar-width: thin;">
    {#each tables as table}
      <div class="bg-white/90 backdrop-blur-sm rounded-lg border border-white/20 p-2 text-center shadow-lg"
        style="width: {cardWidth}px; min-width: {cardWidth}px; max-width: {cardWidth}px; height: {cardHeight}px;">
        <img src={table.image ? `data:image/png;base64,${table.image}` : defaultImage} alt="Immagine card"
          class="rounded-lg w-full object-cover mb-2 cursor-pointer"
          style="height: {imageHeight}px; min-height: {imageHeight}px; max-height: {imageHeight}px;"
          role="button"
          tabindex="0"
          on:click={() => openImageModal(table)}
          on:keydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); openImageModal(table); } }} />
        <h2 class="text-base font-semibold text-gray-900 mb-2 truncate">{table.name}</h2>
        <div class="flex justify-center gap-2">
          <a href="/home/table?table={encodeURIComponent(table.name)}" class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-1 px-2 rounded text-xs flex items-center gap-1">
            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"></path>
            </svg>
            {$_('home.open')}
          </a>
          <button class="bg-red-500 hover:bg-red-700 text-white font-bold py-1 px-2 rounded text-xs flex items-center gap-1" on:click={() => chiediElimina(table.name)}>
            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path>
            </svg>
            {$_('home.delete_table')}
          </button>
        </div>
      </div>
    {/each}

  <!-- IMAGE MANAGEMENT MODAL -->
  {#if showImageModal && selectedTable}
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/40">
      <div class="bg-white rounded-lg shadow-lg p-6 w-full max-w-sm">
        <h2 class="text-lg font-semibold mb-4">{$_('home.set_image')} {selectedTable.name}</h2>
        <div class="mb-4 flex flex-col items-center justify-center gap-2">
          <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" on:click={selectAndSetImage}>
            {$_('home.select_image')}
          </button>
          <button class="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded" on:click={fetchAndSetLogo}>
            {$_('home.download_logo')} {apiService === 'rawg' ? 'RAWG' : 'TheGamesDB'}
          </button>
          {#if selectedTable.image}
            <button class="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded" on:click={deleteImage}>
              {$_('home.delete_image')}
            </button>
          {/if}
        </div>
        <div class="flex justify-end gap-3 mt-4">
          <button class="bg-gray-300 hover:bg-gray-400 text-gray-800 font-bold py-2 px-4 rounded" on:click={() => { showImageModal = false; selectedTable = null; }}>{$_('home.close')}</button>
        </div>
      </div>
    </div>
  {/if}

  <!-- DELETE TABLE MODAL -->
  {#if showDeleteModal && tableToDelete}
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/40">
      <div class="bg-white rounded-lg shadow-lg p-6 w-full max-w-sm">
        <h2 class="text-lg font-semibold mb-4">{$_('home.confirm_delete')}</h2>
        <p class="mb-4">{$_('home.delete_warning', { values: { table: tableToDelete } })}</p>
        <div class="flex justify-end gap-3">
          <button class="bg-gray-300 hover:bg-gray-400 text-gray-800 font-bold py-2 px-4 rounded flex items-center gap-2" on:click={() => { showDeleteModal = false; tableToDelete = null; }}>
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
            </svg>
            {$_('home.cancel')}
          </button>
          <button class="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded flex items-center gap-2" on:click={confermaElimina}>
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path>
            </svg>
            {$_('home.delete')}
          </button>
        </div>
      </div>
    </div>
  {/if}
  </div>
</main>
</div>
