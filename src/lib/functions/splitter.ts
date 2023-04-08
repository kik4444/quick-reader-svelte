/*    
 *    This file is part of Quick Reader.
 *
 *    Quick Reader is free software: you can redistribute it and/or modify
 *    it under the terms of the GNU General Public License as published by
 *    the Free Software Foundation, either version 3 of the License, or
 *    (at your option) any later version.
 *
 *    Quick Reader is distributed in the hope that it will be useful,
 *    but WITHOUT ANY WARRANTY; without even the implied warranty of
 *    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *    GNU General Public License for more details.
 *
 *    You should have received a copy of the GNU General Public License
 *    along with Quick Reader.  If not, see <https://www.gnu.org/licenses/>.
 */

export interface TextChunk {
    chunk: string;
    startPos: number;
    stopPos: number;
}

export default function splitText(input: string, chunkSize: number): TextChunk[] {
    let delimitedText = [] as TextChunk[];

    const matches = input.matchAll(/\S+/g);

    for (const match of matches) {
        delimitedText.push({
            chunk: match[0],
            startPos: match.index!,
            stopPos: match.index! + match[0].length - 1,
        });
    }

    if (chunkSize > 1) {
        let newDelimitedText = [] as TextChunk[];

        for (let i = 0; i < delimitedText.length; i += chunkSize) {
            const sections = delimitedText.slice(i, i + chunkSize);

            const joinedText = sections
                .map(section => section.chunk)
                .reduce((prev, cur) => prev + " " + cur);

            newDelimitedText.push({
                chunk: joinedText,
                startPos: sections[0]!.startPos,
                stopPos: sections[sections.length - 1]!.stopPos
            });
        }

        return newDelimitedText;
    }

    return delimitedText;
}
