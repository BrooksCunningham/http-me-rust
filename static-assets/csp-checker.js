addEventListener("securitypolicyviolation", (event) => {
    console.log(event);
    const violationData = {
        'document-uri': event.documentURI,
        'referrer': event.referrer,
        'violated-directive': event.violatedDirective,
        'effective-directive': event.effectiveDirective,
        'original-policy': event.originalPolicy,
        'blocked-uri': event.blockedURI,
        'status-code': event.statusCode,
    };

    fetch('/csp-violations-reporting-endpoint', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'csp-violation': JSON.stringify(violationData)
        },
        body: JSON.stringify(violationData)
    }).then(response => {
        if (!response.ok) {
            console.error('CSP violation report failed:', response.statusText);
        }
    }).catch(error => {
        console.error('Error sending CSP violation report:', error);
    });
});
