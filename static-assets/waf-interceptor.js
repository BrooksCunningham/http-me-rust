// --- Global Fetch Interceptor for WAF 406 Challenge ---
(function() {
    // Store the original window.fetch function
    const originalFetch = window.fetch; 

    // A flag to prevent multiple reloads from simultaneous fetch calls
    let isReloading = false;

    window.fetch = async function(...args) {
        // Perform the original request
        const response = await originalFetch.apply(this, args);

        // --- THIS IS THE CORE WAF CHALLENGE LOGIC ---
        // Check for 406 status and ensure we aren't already reloading
        if (response.status === 406 && !isReloading) {
            console.log('Global fetch interceptor detected WAF 406!');
            
            const currentUrl = new URL(window.location.href);
            
            // Check if param already exists to avoid a reload loop
            if (!currentUrl.searchParams.has('challenge-client')) {
                // Set flag to prevent other concurrent requests from also
                // triggering a reload
                isReloading = true; 
                console.log('Reloading with ?challenge-client parameter...');
                
                currentUrl.searchParams.set('challenge-client', ''); // Sets param without a value
                window.location.href = currentUrl.toString();

                // Return a promise that never resolves to stop
                // the original caller from processing the 406 response
                // since the page is reloading.
                return new Promise(() => {}); 
            } else {
                console.log('Already on challenge page. WAF block persists.');
            }
        }

        // If not 406, return the response as normal
        return response;
    };
})();
