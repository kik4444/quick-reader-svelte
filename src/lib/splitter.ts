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
    start_pos: number;
    stop_pos: number;
}

export function split_text(input: string, chunk_size: number): TextChunk[] {
    let delimited_text = [] as TextChunk[];

    let matches = input.matchAll(/\S+/g);
    for (const match of matches) {
        delimited_text.push({
            chunk: match[0],
            start_pos: match.index!,
            stop_pos: match.index! + match[0].length - 1,
        });
    }

    if (chunk_size > 1) {
        let new_delimited_text = [] as TextChunk[];

        for (let i = 0; i < delimited_text.length; i += chunk_size) {
            let sections = delimited_text.slice(i, i + chunk_size);

            let joined_text = sections.map(section => section.chunk)
                .reduce((prev, cur) => prev + " " + cur);

            new_delimited_text.push({
                chunk: joined_text,
                start_pos: sections[0].start_pos,
                stop_pos: sections[sections.length - 1].stop_pos
            });
        }

        return new_delimited_text;
    }

    return delimited_text;
}
