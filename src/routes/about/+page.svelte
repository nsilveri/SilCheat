<script>
	import { _ } from 'svelte-i18n';
</script>

<div class="relative min-h-screen flex flex-col" style="min-height: 100vh;">
	<div class="absolute inset-0 w-full h-full" style="background: linear-gradient(120deg, #ffeec9 0%, #ffd6e0 50%, #ffdbce 100%); opacity: 0.8; backdrop-filter: blur(16px); z-index: 0;"></div>
	<header class="w-full pt-5 px-5 fixed top-0 left-0 right-0 z-10 bg-transparent">
		<div class="w-full bg-white/50 backdrop-blur-sm rounded-lg border border-black/50 p-4 sm:p-2 text-center shadow-lg mx-auto">
			<div class="mb-1">
				<h1 class="text-2xl font-semibold text-gray-900 mb-1">{$_('about.title')}</h1>
				<p class="text-gray-700">
					{$_('about.description')}
				</p>
			</div>
		</div>
	</header>
	<main class="flex-grow px-5 pb-8" style="padding-top: 120px; margin-bottom: 45px;">
		<div class="max-w-4xl mx-auto space-y-8">

			<!-- Welcome Section -->
			<div class="bg-white/90 backdrop-blur-sm rounded-lg border border-white/20 p-8 shadow-lg">
				<div class="flex flex-col md:flex-row items-center gap-6">
					<div class="flex-1">
						<h2 class="text-3xl font-bold text-gray-900 mb-4">{$_('about.welcome_title')}</h2>
						<p class="text-lg text-gray-700 leading-relaxed">
							{$_('about.welcome_description')}
						</p>
					</div>
					<div class="flex-shrink-0">
						<!-- Use the public `static/` path so SvelteKit can serve the image in dev and production -->
						<!--
						<img src="/icons/logo.png" alt="logo" class="w-32 h-32 rounded-full object-cover shadow-lg" />
						-->
						</div>
				</div>
			</div>

			<!-- Getting Started -->
			<div class="bg-white/90 backdrop-blur-sm rounded-lg border border-white/20 p-8 shadow-lg">
				<h2 class="text-2xl font-bold text-gray-900 mb-6">{$_('about.getting_started')}</h2>

				<div class="grid md:grid-cols-2 gap-6">
					<div class="space-y-6">
						<div class="flex items-start gap-4">
							<div class="flex-shrink-0 w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center">
								<svg class="w-6 h-6 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"></path>
								</svg>
							</div>
							<div>
								<h3 class="text-xl font-semibold text-gray-800 mb-3">{$_('about.import_files_title')}</h3>
								<p class="text-gray-600 leading-relaxed">
									{$_('about.import_files_description')}
								</p>
							</div>
						</div>

						<div class="flex items-start gap-4">
							<div class="flex-shrink-0 w-12 h-12 bg-green-100 rounded-lg flex items-center justify-center">
								<svg class="w-6 h-6 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"></path>
								</svg>
							</div>
							<div>
								<h3 class="text-xl font-semibold text-gray-800 mb-3">{$_('about.manage_tables_title')}</h3>
								<p class="text-gray-600 leading-relaxed">
									{$_('about.manage_tables_description')}
								</p>
							</div>
						</div>

						<div class="flex items-start gap-4">
							<div class="flex-shrink-0 w-12 h-12 bg-purple-100 rounded-lg flex items-center justify-center">
								<svg class="w-6 h-6 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
								</svg>
							</div>
							<div>
								<h3 class="text-xl font-semibold text-gray-800 mb-3">{$_('about.create_records_title')}</h3>
								<p class="text-gray-600 leading-relaxed">
									{$_('about.create_records_description')}
								</p>
							</div>
						</div>
					</div>

					<div class="space-y-6">
						<div class="flex items-start gap-4">
							<div class="flex-shrink-0 w-12 h-12 bg-yellow-100 rounded-lg flex items-center justify-center">
								<svg class="w-6 h-6 text-yellow-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path>
								</svg>
							</div>
							<div>
								<h3 class="text-xl font-semibold text-gray-800 mb-3">{$_('about.edit_records_title')}</h3>
								<p class="text-gray-600 leading-relaxed">
									{$_('about.edit_records_description')}
								</p>
							</div>
						</div>

						<div class="flex items-start gap-4">
							<div class="flex-shrink-0 w-12 h-12 bg-red-100 rounded-lg flex items-center justify-center">
								<svg class="w-6 h-6 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
								</svg>
							</div>
							<div>
								<h3 class="text-xl font-semibold text-gray-800 mb-3">{$_('about.export_data_title')}</h3>
								<p class="text-gray-600 leading-relaxed">
									{$_('about.export_data_description')}
								</p>
							</div>
						</div>

						<div class="flex items-start gap-4">
							<div class="flex-shrink-0 w-12 h-12 bg-indigo-100 rounded-lg flex items-center justify-center">
								<svg class="w-6 h-6 text-indigo-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
								</svg>
							</div>
							<div>
								<h3 class="text-xl font-semibold text-gray-800 mb-3">{$_('about.settings_title')}</h3>
								<p class="text-gray-600 leading-relaxed">
									{$_('about.settings_description')}
								</p>
							</div>
						</div>
					</div>
				</div>
			</div>

			<!-- Support -->
			<div class="bg-white/90 backdrop-blur-sm rounded-lg border border-white/20 p-8 shadow-lg">
				<div class="flex flex-col md:flex-row items-center gap-6">
					<div class="flex-1">
						<h2 class="text-2xl font-bold text-gray-900 mb-4">{$_('about.support')}</h2>
						<p class="text-gray-700 leading-relaxed">
							{$_('about.support_description')}
						</p>
					</div>
					<div class="flex-shrink-0">
						<div class="w-24 h-24 bg-gradient-to-br from-green-500 to-blue-600 rounded-full flex items-center justify-center shadow-lg">
							<svg class="w-12 h-12 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18.364 5.636l-3.536 3.536m0 5.656l3.536 3.536M9.172 9.172L5.636 5.636m3.536 9.192L5.636 18.364M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-5 0a4 4 0 11-8 0 4 4 0 018 0z"></path>
							</svg>
						</div>
					</div>
				</div>
			</div>

		</div>
	</main>
</div>
