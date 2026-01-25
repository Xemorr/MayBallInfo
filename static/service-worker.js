self.addEventListener('install', (event) => {
    // Perform install steps
    event.waitUntil(
        caches.open('MAY-BALL')
            .then((cache) => {
                return cache.addAll([
                    '/',
                    '/index.html',
                    '/styles-redesign.css',
                    '/styles-redesign.css',
                    // Add other assets and pages you want to cache
                ]);
            })
    );
});

self.addEventListener('activate', (event) => {
    const cacheWhitelist = ['MAY-BALL']; // List of cache versions to keep
    event.waitUntil(
        caches.keys().then((cacheNames) => {
            return Promise.all(
                cacheNames.map((cacheName) => {
                    if (!cacheWhitelist.includes(cacheName)) {
                        return caches.delete(cacheName);
                    }
                })
            );
        })
    );
});

self.addEventListener('fetch', (event) => {
    event.respondWith(
        fetch(event.request).then((response) => {
            if (response && response.status === 200) {
                const responseClone = response.clone();
                caches.open('MATURITY-TEST-7').then((cache) => {
                    cache.put(event.request, responseClone);
                });
            }
            return response;
        }).catch(() => {
            return caches.match(event.request)
        })
    );
});