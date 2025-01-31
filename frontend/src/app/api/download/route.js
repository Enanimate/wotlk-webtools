import { NextRequest, NextResponse } from 'next/server';
import fs from 'fs';
import path from 'path';

export async function GET() {
    let response = await fetch('http://127.0.0.1:3001/', {
        method: 'GET',
        headers: {
            'Content-type': 'application/octet-stream'
        }
    })
    .then(result => {
        console.log(result)
    })

    return NextResponse.json({ status: 200, message: "Received", data: response })
}