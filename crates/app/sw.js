// Network first, so a new deploy shows up as soon as the phone is online,
// falling back to the last copy saved here so the game also works offline.
const CACHE = "sudoku-for-mom";

self.addEventListener("install", () => self.skipWaiting());
self.addEventListener("activate", (event) => event.waitUntil(self.clients.claim()));

self.addEventListener("fetch", (event) => {
  if (event.request.method !== "GET" || !event.request.url.startsWith(self.location.origin)) return;
  event.respondWith(
    caches.open(CACHE).then(async (cache) => {
      try {
        const response = await fetch(event.request);
        if (response.ok) await cache.put(event.request, response.clone());
        return response;
      } catch (error) {
        const cached = await cache.match(event.request, { ignoreSearch: true });
        if (cached) return cached;
        throw error;
      }
    }),
  );
});
