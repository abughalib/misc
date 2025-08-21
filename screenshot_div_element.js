/**
* How to use it?
* Paste the script into the console tab after opening Inspect/Developer Tools (F12 -> Most Chromimum)
* Execute this: screenshotElements(selector, options) // if selector is a class then then use screenshotElements(".<class_name>", options)
* Or In case of id, screenshotElements("#<id>", options)
* 
* Auto Download Screenshot: screenshotElements('.myClass', { openInTab: false });
**/

(async function() {
    if (typeof html2canvas === 'undefined') {
        const script = document.createElement('script');
        script.src = 'https://cdnjs.cloudflare.com/ajax/libs/html2canvas/1.4.1/html2canvas.min.js';
        document.head.appendChild(script);
        await new Promise(resolve => script.onload = resolve);
    }

    window.screenshotElements = async function(selector, { openInTab = true } = {}) {
        const elements = document.querySelectorAll(selector);
        if (!elements.length) {
            console.error('No elements found for selector:', selector);
            return;
        }

        for (let i = 0; i < elements.length; i++) {
            const element = elements[i];

            try {
                const canvas = await html2canvas(element, {
                    useCORS: true,
                    backgroundColor: null,
                    scale: 2
                });

                const dataURL = canvas.toDataURL('image/png');

                if (openInTab) {
                    const newTab = window.open();
                    if (newTab) {
                        newTab.document.body.innerHTML = `<img src="${dataURL}" style="max-width: 100%;" />`;
                    } else {
                        console.warn('Pop-up blocked. Falling back to download.');
                        downloadImage(dataURL, `screenshot-${i + 1}.png`);
                    }
                } else {
                    downloadImage(dataURL, `screenshot-${i + 1}.png`);
                }

            } catch (err) {
                console.error(`Error capturing element ${i + 1}:`, err);
            }
        }

        function downloadImage(dataURL, filename) {
            const link = document.createElement('a');
            link.href = dataURL;
            link.download = filename;
            document.body.appendChild(link);
            link.click();
            document.body.removeChild(link);
        }
    };

    console.log("Function `screenshotElements(selector, options)` is ready.");
    console.log("Example: screenshotElements('.card') or screenshotElements('.card', { openInTab: false })");
})();
