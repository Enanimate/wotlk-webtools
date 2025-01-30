"use client";

export default function Home() {
  const downloadFile = async () => {
    const response = await fetch("/api/download");
    const blob = await response.blob();
    const url = window.URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.setAttribute('download', 'wotlk-client.zip');
    document.body.appendChild(link);
    link.click();
  };

  return (
    <>
      <button onClick={downloadFile}>download</button>
    </>
  );
}
