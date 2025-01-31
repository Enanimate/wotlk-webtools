"use client";

export default function download() {
    const downloadFile = async (event) => {
        event.preventDefault()

    
        let response = await fetch('/api/download', {
          method: 'GET',
          headers: {
            'Content-type': 'application/octet-stream'
          }
        })
      };

    return (
        <button onClick={downloadFile}>download</button>
    );
}