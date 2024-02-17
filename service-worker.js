self.addEventListener('install', (event) => {
    // Perform install steps
    event.waitUntil(
        caches.open('YOUR_CACHE_NAME')
            .then((cache) => {
                return cache.addAll([
                    '/',
                    '/index.html',
                    '/styles.css',
                    // Add other assets and pages you want to cache
                ]);
            })
    );
});

self.addEventListener('fetch', (event) => {
    event.respondWith(
        caches.match(event.request)
            .then((response) => {
                // Cache hit - return response
                if (response) {
                    return response;
                }
                return fetch(event.request);
            }
        )
    );
});
