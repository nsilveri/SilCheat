<script>
  import { onMount, afterUpdate } from 'svelte';
  import { page } from '$app/stores';
  import { invoke } from '@tauri-apps/api/core';
  import { save } from '@tauri-apps/plugin-dialog';
  import { _ } from 'svelte-i18n';
  import Sortable from 'sortablejs';

  let items = [];
  let fields = [];
  let listEl = null;
  let tableName = '';
  let sortableInstance = null;

  let showViewModal = false;
  let selectedItem = null;
  let showToast = false;
  let toastMsg = '';
  let toastType = 'success';
  let showDeleteRecordModal = false;
  let recordToDelete = null;
  let selectedIndex = -1;
  let showBulkModal = false;
  let bulkText = '';
  let bulkImporting = false;
  let showAddMenu = false;
  let addMenuEl = null;

  function initializeSortable() {
  const list = listEl;
  console.log('Initializing sortable list:', list);

  if (!list || items.length <= 1) {
    if (sortableInstance) {
      try { sortableInstance.destroy(); } catch {}
      sortableInstance = null;
    }
    return;
  }

  // Se già esiste, distruggi la precedente istanza
  if (sortableInstance) {
    try { sortableInstance.destroy(); } catch {}
    sortableInstance = null;
  }

  // Crea una nuova istanza SortableJS
  sortableInstance = Sortable.create(list, {
    animation: 150,
    handle: '.drag-handle',
    draggable: 'li.sortable-item',
    ghostClass: 'sortable-ghost',
    chosenClass: 'sortable-chosen',
    dragClass: 'sortable-drag',
    fallbackOnBody: true,

    onStart: (evt) => {
      console.log('Drag started:', evt.oldIndex);
    },

    onEnd: async (evt) => {
      const { oldIndex, newIndex } = evt;
      console.log('Drag ended:', oldIndex, '→', newIndex);

      if (oldIndex === undefined || newIndex === undefined || oldIndex === newIndex) {
        console.log('No movement detected, skipping reorder.');
        return;
      }

      // Aggiorna l’array items in modo reattivo
      const newItems = [...items];
      const [moved] = newItems.splice(oldIndex, 1);
      newItems.splice(newIndex, 0, moved);
      items = newItems;

      // Salva l’ordine aggiornato nel backend
        try {
          const recordOrder = items.map(i => i.id);
          await invoke('update_record_order', { tableName, recordOrder });
          toastMsg = $_('table.order_updated');
          toastType = 'success';
        } catch (e) {
          console.error('Errore aggiornando ordine:', e);
          toastMsg = $_('table.order_update_error');
          toastType = 'error';
        }

      showToast = true;
      setTimeout(() => (showToast = false), 2500);
    },
  });
}


  import { onDestroy } from 'svelte';
  onDestroy(() => {
    if (sortableInstance) {
      try { sortableInstance.destroy(); } catch {}
      sortableInstance = null;
    }
  });

  onMount(async () => {
  const urlParams = $page.url.searchParams;
  tableName = urlParams.get('table') || '';
  console.log('Table name:', tableName);

  if (!tableName) {
    console.log('No table name in URL');
    return;
  }

  try {
    items = await invoke('get_records', { tableName });
    console.log('Loaded items:', items);

    if (items.length > 0) {
      const allFields = Object.keys(items[0]).filter(f => f !== 'id');
      const otherFields = allFields.filter(f => !['desc', 'code', 'enable'].includes(f));
      fields = ['desc', 'code', 'enable'].filter(f => allFields.includes(f));
      if (otherFields.length > 0) {
        fields.push('others');
        items = items.map(item => ({ ...item, others: otherFields.length.toString() }));
      }
    }

    console.log('Fields:', fields);
    initializeSortable();
  } catch (e) {
    console.error('Errore caricamento record:', e);
  }
});

// close add menu on outside click
onMount(() => {
  const outsideHandler = (e) => {
    if (showAddMenu && addMenuEl && !addMenuEl.contains(e.target)) {
      showAddMenu = false;
    }
  };
  window.addEventListener('click', outsideHandler);
  return () => window.removeEventListener('click', outsideHandler);
});

  //afterUpdate(() => {
  //  if (items.length > 1) initializeSortable();
  //});

  function openViewModal(item) {
    selectedItem = item;
    showViewModal = true;
  }

  function selectRow(index) {
    if (selectedIndex === index) {
      selectedIndex = -1;
    } else {
      selectedIndex = index;
    }
  }

  function openDeleteRecordModal(item) {
    recordToDelete = item;
    showDeleteRecordModal = true;
  }

  function parseCheats(text) {
    const norm = text.replace(/\r\n/g, '\n').trim();
    const results = [];

    // Try labeled pairs: desc: ...\ncode: ... (supports multiple blocks)
    const labelRegex = /(?:desc|description)[:\- ]*(.+?)\n(?:code|cheat)[:\- ]*([\s\S]*?)(?=\n(?:desc|description)[:\- ]|\n*$)/igm;
    let m;
    while ((m = labelRegex.exec(norm)) !== null) {
      results.push({ desc: m[1].trim(), code: m[2].trim() });
    }
    if (results.length) return results;

    // Try blocks separated by blank lines: first line = desc, rest = code
    const blocks = norm.split(/\n\s*\n/);
    if (blocks.length > 1) {
      for (const b of blocks) {
        const lines = b.split('\n').map(l => l.trim()).filter(Boolean);
        if (lines.length >= 2) {
          results.push({ desc: lines[0], code: lines.slice(1).join('\n') });
        }
      }
      if (results.length) return results;
    }

    // Heuristic: if every non-empty line looks like "desc <code>" on the same line,
    // parse each line as a separate cheat by splitting on the last whitespace.
    // We require the last token to contain a digit (simple code detection) and
    // to be at least 3 chars long to avoid false positives.
    const lines = norm.split('\n').map(l => l.trim()).filter(Boolean);
    if (lines.length > 1) {
      const allLikelyLinePairs = lines.every(l => {
        const parts = l.split(/\s+/);
        const last = parts[parts.length - 1] || '';
        return last.length >= 3 && /\d/.test(last) && parts.length >= 2;
      });

      if (allLikelyLinePairs) {
        for (const l of lines) {
          const idx = l.lastIndexOf(' ');
          if (idx === -1) continue;
          const d = l.slice(0, idx).trim();
          const c = l.slice(idx + 1).trim();
          if (d) results.push({ desc: d, code: c });
        }
        if (results.length) return results;
      }
    }

    // Fallback: alternating lines (desc, code, desc, code...)
    for (let i = 0; i < lines.length; i += 2) {
      const d = lines[i];
      const c = lines[i + 1] || '';
      if (d) results.push({ desc: d, code: c });
    }

    return results;
  }

  async function importBulk() {
    if (!bulkText || !bulkText.trim()) {
      toastMsg = $_('bulk_add.error').replace('{err}', 'No input');
      toastType = 'error';
      showToast = true;
      setTimeout(() => (showToast = false), 3000);
      return;
    }

    bulkImporting = true;
    try {
      const parsed = parseCheats(bulkText);
      if (!parsed || parsed.length === 0) {
        throw new Error('No cheats parsed from input');
      }

      // Get table columns to build full record objects
      const cols = await invoke('get_table_columns', { tableName });
      const usableCols = cols.filter(c => c !== 'id' && c !== 'image');

      let created = 0;
      for (const p of parsed) {
        const record = {};
        for (const f of usableCols) {
          if (f === 'enable' || f === 'abilitato' || f === 'attivo') {
            record[f] = 'false';
          } else if (f === 'desc') {
            record[f] = p.desc || '';
          } else if (f === 'code') {
            record[f] = p.code || '';
          } else {
            record[f] = '';
          }
        }
        try {
          await invoke('insert_record', { tableName, record });
          created++;
        } catch (e) {
          console.error('Error inserting record:', e);
          // continue with others
        }
      }

      toastMsg = $_('bulk_add.success').replace('{count}', String(created));
      toastType = 'success';

      // Reload items
      items = await invoke('get_records', { tableName });
      initializeSortable();
      showBulkModal = false;
      bulkText = '';
    } catch (e) {
      console.error('Bulk import error:', e);
      toastMsg = $_('bulk_add.error').replace('{err}', String(e));
      toastType = 'error';
    } finally {
      bulkImporting = false;
      showToast = true;
      setTimeout(() => (showToast = false), 3500);
    }
  }

  async function confermaDeleteRecord() {
    if (!recordToDelete) return;
    try {
      await invoke('delete_record', { tableName, id: recordToDelete.id });
      items = items.filter(i => i !== recordToDelete);
      toastMsg = `Record "${recordToDelete.desc}" eliminato`;
      toastType = 'success';
    } catch (e) {
      toastMsg = 'Errore eliminando il record';
      toastType = 'error';
    }
    showToast = true;
    setTimeout(() => (showToast = false), 2500);
    showDeleteRecordModal = false;
    recordToDelete = null;
  }

  async function exportCHT() {
    try {
      const filePath = await save({
        filters: [{ name: 'File CHT', extensions: ['cht'] }],
        defaultPath: `${tableName}.cht`
      });
      if (!filePath) return;
      const result = await invoke('export_cht_to_path', { tableName, filePath });
      toastMsg = result;
      toastType = 'success';
    } catch (e) {
      toastMsg = 'Errore export: ' + e;
      toastType = 'error';
    }
    showToast = true;
    setTimeout(() => (showToast = false), 2500);
  }

  $: document.body.style.overflow = showViewModal ? 'hidden' : '';

  // Move row up by index
  async function moveRowUp(index) {
    if (index <= 0) return;
    const newItems = items.slice();
    const [moved] = newItems.splice(index, 1);
    newItems.splice(index - 1, 0, moved);
    items = newItems;

    try {
      const recordOrder = items.map(i => i.id);
      await invoke('update_record_order', { tableName, recordOrder });
      toastMsg = $_('table.order_updated');
      toastType = 'success';
    } catch (e) {
      console.error(e);
      toastMsg = $_('table.order_update_error');
      toastType = 'error';
    }
    showToast = true;
    setTimeout(() => (showToast = false), 2000);
  }

  // Move row down by index
  async function moveRowDown(index) {
    if (index >= items.length - 1) return;
    const newItems = items.slice();
    const [moved] = newItems.splice(index, 1);
    newItems.splice(index + 1, 0, moved);
    items = newItems;

    try {
      const recordOrder = items.map(i => i.id);
      await invoke('update_record_order', { tableName, recordOrder });
      toastMsg = $_('table.order_updated');
      toastType = 'success';
    } catch (e) {
      console.error(e);
      toastMsg = $_('table.order_update_error');
      toastType = 'error';
    }
    showToast = true;
    setTimeout(() => (showToast = false), 2000);
  }
</script>

<div class="min-h-screen flex flex-col bg-gradient-to-br from-cyan-100 via-indigo-100 to-blue-100">
  <!-- HEADER -->
  <header class="w-full pt-5 px-5 fixed top-0 left-0 z-10 bg-transparent">
    <div class="w-full bg-white/50 backdrop-blur-sm rounded-lg border border-black/50 p-4 sm:p-2 shadow-lg flex items-center justify-between">
      <button class="bg-gray-200 hover:bg-gray-300 text-gray-800 font-bold py-2 px-4 rounded" on:click={() => window.history.back()}>
        ← {$_('home.back')}
      </button>
      
      <div class="text-center flex-1">
        <h1 class="text-2xl font-semibold text-gray-900 mb-1">{$_('home.table_title')} {tableName}</h1>
        <p class="text-gray-700 text-sm">{$_('home.table_description')}</p>
      </div>
      <div class="flex gap-2 items-center">
        <button class="bg-green-500 hover:bg-green-600 text-white font-bold py-3 px-4 rounded text-xs" class:opacity-50={selectedIndex <= 0} on:click={async () => {
          if (selectedIndex <= 0) {
            toastMsg = $_('table.select_cheat_to_move');
            toastType = 'warning';
            showToast = true;
            setTimeout(() => (showToast = false), 5000);
            return;
          }
          await moveRowUp(selectedIndex);
          selectedIndex = Math.max(0, selectedIndex - 1);
        }} title="Move selected up">▲</button>
        <button class="bg-green-500 hover:bg-green-600 text-white font-bold py-3 px-4 rounded text-xs" class:opacity-50={selectedIndex < 0 || selectedIndex >= items.length -1} on:click={async () => {
          if (selectedIndex < 0 || selectedIndex >= items.length -1) {
            toastMsg = $_('table.select_cheat_to_move');
            toastType = 'warning';
            showToast = true;
            setTimeout(() => (showToast = false), 5000);
            return;
          }
          await moveRowDown(selectedIndex);
          selectedIndex = Math.min(items.length -1, selectedIndex + 1);
        }} title="Move selected down">▼</button>
        <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" on:click={exportCHT}>
          {$_('home.export_cht')}
        </button>
        <div class="relative" bind:this={addMenuEl}>
          <button class="bg-green-500 hover:bg-green-600 text-white font-bold py-2 px-4 rounded flex items-center gap-2" on:click|stopPropagation={() => { showAddMenu = !showAddMenu; }}>
            {$_('home.add')}
            <svg class="w-4 h-4" viewBox="0 0 20 20" fill="none" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 8l4 4 4-4"/></svg>
          </button>
          {#if showAddMenu}
            <div class="absolute right-0 mt-2 w-44 bg-white rounded shadow-lg z-20 border border-gray-200">
              <a href="/home/table/create?table={encodeURIComponent(tableName)}" class="block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100" on:click={() => { showAddMenu = false; }}>
                {$_('add_one')}
              </a>
              <button class="w-full text-left px-4 py-2 text-sm text-gray-700 hover:bg-gray-100" on:click={() => { showBulkModal = true; showAddMenu = false; }}>
                {$_('magic_add')}
              </button>
            </div>
          {/if}
        </div>
      </div>
    </div>
  </header>

  <!-- MAIN -->
  <main class="flex-grow px-5 pt-28 mb-24">
    <div class="w-full bg-white/90 backdrop-blur-sm rounded-lg border border-white/20 p-6 shadow-lg">
      <!-- TABLE HEADER -->
      <div class="flex gap-4 py-2 font-bold text-gray-500">
        <span class="w-8"></span>
        {#each fields as field}
          <span class="flex-1 {field === 'enable' ? 'text-center' : ''}">{field === 'enable' ? $_('home.enabled') : field}</span>
        {/each}
        <span class="w-1/4 text-center">{$_('home.actions')}</span>
      </div>

  <!-- TABLE ROWS -->
  <ul class="sortable-list" bind:this={listEl}>
        {#each items as item, i (item.id)}
          <li class="flex gap-4 py-2 items-center cursor-move hover:bg-gray-50 sortable-item {selectedIndex === i ? 'bg-blue-50 border-l-4 border-blue-400' : ''}"
              data-id={item.id} on:click={() => selectRow(i)}>
            <span class="drag-handle w-8 flex items-center justify-center cursor-move text-gray-400 hover:text-gray-600">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 8h16M4 16h16"></path>
              </svg>
            </span>
            {#each fields as field}
              {#if field === 'enable'}
                <span class="flex-1 flex justify-center">
                  <input type="checkbox" checked={item[field] === 'true' || item[field] === '1'} disabled />
                </span>
              {:else}
                <span class="flex-1 truncate">{item[field]}</span>
              {/if}
            {/each}
              <span class="w-1/4 flex gap-2 justify-center">
              <button class="bg-blue-500 hover:bg-blue-600 text-white font-bold py-1 px-2 rounded text-xs" on:click={() => openViewModal(item)}>
                {$_('home.view')}
              </button>
              <a href="/home/table/edit?table={encodeURIComponent(tableName)}&id={encodeURIComponent(item.id)}" class="bg-yellow-400 hover:bg-yellow-500 text-white font-bold py-1 px-2 rounded text-xs">
                {$_('home.edit')}
              </a>
                <button class="bg-green-500 hover:bg-green-600 text-white font-bold py-1 px-2 rounded text-xs" on:click={() => moveRowUp(i)} aria-label="Move up" title="Move up" class:opacity-50={i===0} disabled={i===0}>
                  ↑
                </button>
                <button class="bg-green-500 hover:bg-green-600 text-white font-bold py-1 px-2 rounded text-xs" on:click={() => moveRowDown(i)} aria-label="Move down" title="Move down" class:opacity-50={i===items.length-1} disabled={i===items.length-1}>
                  ↓
                </button>
                <button class="bg-red-500 hover:bg-red-700 text-white font-bold py-1 px-2 rounded text-xs" on:click={() => openDeleteRecordModal(item)}>
                  {$_('home.delete')}
                </button>
            </span>
          </li>
        {/each}
      </ul>
    </div>
  </main>

  <!-- VIEW MODAL -->
  {#if showViewModal && selectedItem}
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/40">
      <div class="bg-white rounded-lg shadow-lg w-full max-w-md flex flex-col max-h-[80vh]">
        <div class="p-6 border-b">
          <h2 class="text-lg font-semibold">{selectedItem.desc || $_('home.record_details')}</h2>
        </div>
        <div class="flex-1 overflow-y-auto p-6">
          {#each Object.entries(selectedItem).filter(([k]) => !['desc','others','id','order_index'].includes(k)) as [key,value]}
            <div class="flex justify-between">
              <span class="font-medium">{key}:</span>
              <span>{value}</span>
            </div>
          {/each}
        </div>
        <div class="p-6 border-t flex justify-end">
          <button class="bg-gray-300 hover:bg-gray-400 text-gray-800 font-bold py-2 px-4 rounded" on:click={() => { showViewModal = false; selectedItem = null; }}>
            {$_('home.close')}
          </button>
        </div>
      </div>
    </div>
  {/if}

  <!-- DELETE MODAL -->
  {#if showDeleteRecordModal && recordToDelete}
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/40">
      <div class="bg-white rounded-lg shadow-lg p-6 w-full max-w-sm">
        <h2 class="text-lg font-semibold mb-4">{$_('home.confirm_delete')}</h2>
        <p class="mb-4">{$_('table.delete_warning')}</p>
        <div class="flex justify-end gap-3">
          <button class="bg-gray-300 hover:bg-gray-400 text-gray-800 font-bold py-2 px-4 rounded" on:click={() => { showDeleteRecordModal = false; recordToDelete = null; }}>
            {$_('home.cancel')}
          </button>
          <button class="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded" on:click={confermaDeleteRecord}>
            {$_('home.delete')}
          </button>
        </div>
      </div>
    </div>
  {/if}

  <!-- BULK ADD MODAL -->
  {#if showBulkModal}
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/40">
      <div class="bg-white rounded-lg shadow-lg w-full max-w-2xl flex flex-col max-h-[80vh]">
        <div class="p-4 border-b">
          <h2 class="text-lg font-semibold">{$_('bulk_add.title')}</h2>
        </div>
        <div class="flex-1 overflow-y-auto p-4">
          <textarea bind:value={bulkText} placeholder={$_('bulk_add.placeholder')} class="w-full h-64 border rounded p-3 text-sm"></textarea>
        </div>
        <div class="p-4 border-t flex justify-end gap-3">
          <button class="bg-gray-300 hover:bg-gray-400 text-gray-800 font-bold py-2 px-4 rounded" on:click={() => { showBulkModal = false; bulkText = ''; }}>
            {$_('bulk_add.cancel')}
          </button>
          <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" on:click={importBulk} disabled={bulkImporting}>
            {bulkImporting ? '...' : $_('bulk_add.import')}
          </button>
        </div>
      </div>
    </div>
  {/if}

  <!-- TOAST -->
  {#if showToast}
    <div class="fixed bottom-8 right-8 z-50 px-6 py-3 rounded shadow-lg animate-fadein font-semibold text-white"
      style="background-color: {toastType === 'success' ? '#22c55e' : toastType === 'warning' ? '#f59e0b' : '#ef4444'};">
      {toastMsg}
    </div>
  {/if}
</div>

<style>
@keyframes fadein {
  from { opacity: 0; transform: translateY(20px); }
  to { opacity: 1; transform: translateY(0); }
}
.animate-fadein { animation: fadein 0.3s; }

.sortable-ghost { opacity: 0.4; background-color: #f3f4f6; border: 2px dashed #9ca3af; }
.sortable-chosen { background-color: #e5e7eb; }
.sortable-drag { transform: rotate(5deg); box-shadow: 0 4px 12px rgba(0,0,0,0.15); }
</style>
