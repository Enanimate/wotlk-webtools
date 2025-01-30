import { NextRequest, NextResponse } from 'next/server';
import fs from 'fs';
import path from 'path';

export async function GET() {
    const fileName = "wotlk-client.zip";
    const filePath = path.join(process.cwd(), '../../wotlk-client-file', fileName);

    try {
        const file = fs.readFileSync(filePath);

        return new NextResponse(file, {
            status: 200,
            headers: {
                'Content-Type': 'application/octet-stream', // Adjust content type as needed
                'Content-Disposition': `attachment; filename=${fileName}`,
            },
        });
    } catch (error) {
        return new NextResponse(error, { status: 404 });
    }
}