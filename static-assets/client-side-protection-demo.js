// client-side-protection-demo.js

(function() { // Wrap in an IIFE for best practice

    const form = document.getElementById('credit-card-form');
    const cardNumberInput = document.getElementById('card-number');
    const expiryMonthInput = document.getElementById('expiry-month');
    const expiryYearInput = document.getElementById('expiry-year');
    const cvvInput = document.getElementById('cvv');
    const cardholderNameInput = document.getElementById('cardholder-name');

    const cardNumberError = document.getElementById('card-number-error');
    const expiryDateError = document.getElementById('expiry-date-error');
    const cvvError = document.getElementById('cvv-error');
    const cardholderNameError = document.getElementById('cardholder-name-error');


    // Populate Expiry Years (dynamically)
    const currentYear = new Date().getFullYear() % 100;
    for (let i = 0; i < 10; i++) {
        const year = currentYear + i;
        const yearStr = String(year).padStart(2, '0');
        const option = document.createElement('option');
        option.value = yearStr;
        option.textContent = yearStr;
        expiryYearInput.appendChild(option);
    }


    // Form Submission Handler
    if (form) { // Check if the form exists (good practice)
      form.addEventListener('submit', (event) => {
          let isValid = true;

          // Clear previous errors
          cardNumberError.textContent = '';
          expiryDateError.textContent = '';
          cvvError.textContent = '';
          cardholderNameError.textContent = '';

          // Card Number Validation (Basic)
          const cardNumber = cardNumberInput.value.replace(/\s/g, ''); // Remove spaces
          if (!/^\d{13,19}$/.test(cardNumber)) {
              cardNumberError.textContent = 'Invalid card number';
              isValid = false;
          }

          // Expiry Date Validation
          const expiryMonth = expiryMonthInput.value;
          const expiryYear = expiryYearInput.value;
          const currentYear = new Date().getFullYear() % 100;
          const currentMonth = new Date().getMonth() + 1;

          if (!expiryMonth || !expiryYear) {
              expiryDateError.textContent = "Please select month and year";
              isValid = false;
          } else if (parseInt(expiryYear) < currentYear || (parseInt(expiryYear) === currentYear && parseInt(expiryMonth) < currentMonth)) {
              expiryDateError.textContent = 'Card has expired';
              isValid = false;
          }


          // CVV Validation
          const cvv = cvvInput.value;
          if (!/^[0-9]{3,4}$/.test(cvv)) {
              cvvError.textContent = 'Invalid CVV';
              isValid = false;
          }

          // Cardholder Name Validation (Basic)
          const cardholderName = cardholderNameInput.value.trim();
          if (cardholderName === '') {
              cardholderNameError.textContent = 'Cardholder name is required';
              isValid = false;
          }

          if (!isValid) {
              event.preventDefault();
          }
      });
    }

    // Card Number Formatting
    if(cardNumberInput){
      cardNumberInput.addEventListener('input', (event) => {
        let value = event.target.value.replace(/\D/g, '');
        let formattedValue = '';
        for (let i = 0; i < value.length; i++) {
          if (i > 0 && i % 4 === 0) {
            formattedValue += ' ';
          }
          formattedValue += value[i];
        }
          event.target.value = formattedValue;
      });
    }
    //CVV Input
    if (cvvInput) {
        cvvInput.addEventListener('input', (event) => {
            event.target.value = event.target.value.replace(/\D/g, '');
        });
    }
})();
