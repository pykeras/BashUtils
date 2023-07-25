import aiohttp
import asyncio
import os, json, sys
from aiohttp import ClientSession
from aiofiles import open as aopen
from bs4 import BeautifulSoup
from collections import defaultdict


class Colors:
    OKGREEN = "\033[32m"
    WARNING = "\033[33m"
    FAIL = "\033[31m"
    BOLD = "\033[1m"
    END = "\033[0m"


def clean_filename(filename):
    return (
        filename.replace("%20", "_")
        .replace("%5B", "")
        .replace("%5D", "")
        .replace("_-_", "-")
        .replace("_[320]", "_320")
    )


def extract_artist_name(meta):
    artist_name = meta.split("/")[-1].split(".")[0].lower()
    for suffix in [
        "-songs-coll",
        "-songs-colletion",
        "-songs-collection",
        "-song-collection",
        "-song-colletion",
        "-music-colletion",
        "-music-collection",
        "-best-music",
        "-best-songs",
    ]:
        artist_name = artist_name.split(suffix)[0]
    return artist_name.replace("-", " ").title().replace(" ", "_")


async def fetch_main_page(session: ClientSession, url: str):
    async with session.get(url) as response:
        assert response.status == 200
        return await response.content.read()


async def fetch_music(session: ClientSession, artist_dir: str, url: str):
    async with session.get(url) as response:
        assert response.status == 200
        fname = clean_filename(url.split("/")[-1])
        file_path = os.path.join(artist_dir, fname)
        async with aopen(file_path, "wb") as f:
            await f.write(await response.read())


async def main(quality):
    main_dir = os.path.dirname(os.path.abspath(__file__))
    urls_file = os.path.join(main_dir, "urls.txt")
    try:
        print(f"{Colors.OKGREEN}Reading best of urls file ...{Colors.END}")
        with open(urls_file, "rt", encoding="utf-8") as f:
            urls = tuple(set(f.readlines()))
    except FileNotFoundError as ex:
        print(f"{Colors.FAIL}{urls_file} does not exist{Colors.END}\n")
        sys.exit(1)

    best_of_json = defaultdict(list)
    async with aiohttp.ClientSession() as session:
        print(f"{Colors.OKGREEN}start sending requests ...{Colors.END}\n")
        all_requests = [fetch_main_page(session, url) for url in urls]
        for finished_task in asyncio.as_completed(all_requests):
            content = await finished_task
            try:
                content = BeautifulSoup(content, "html.parser")
                meta = content.select_one("meta[property='og:image']")["content"]
                artist_name = extract_artist_name(meta)
                print(
                    f"{Colors.WARNING}start sending requests for {artist_name} urls ...{Colors.END}"
                )
                match quality:
                    case "320":
                        tracks_urls = [
                            url.select_one("a.icon.dwl")["href"]
                            for url in content.select(".atn~ td+ td")
                        ]
                    case "128":
                        tracks_urls = [
                            url.select_one("a.icon.dwl")["href"]
                            for url in content.select(".atn+ td")
                        ]
                artist_dir = os.path.join(main_dir, f"musics/{artist_name}")
                os.makedirs(artist_dir, exist_ok=True)
                all_musics = [
                    fetch_music(session, artist_dir, url) for url in tracks_urls
                ]
                try:
                    await asyncio.gather(*all_musics, return_exceptions=True)
                except Exception as ex:
                    print(ex)

                best_of_json["artists"].append(
                    {"name": artist_name, "musics_urls": tracks_urls}
                )

            except Exception as ex:
                print(ex)

    out_json = os.path.join(main_dir, "best_of_urls_downloaded.json")
    with open(out_json, "wt") as jf:
        jf.write(json.dumps(best_of_json, indent=2))


if __name__ == "__main__":
    try:
        quality = sys.argv[1]
        if quality not in ["128", "320"]:
            print(
                f"\n{Colors.WARNING}Invalid quality value. Switching to default (320).{Colors.END}"
            )
            quality = "320"
    except IndexError:
        quality = "320"
    print(f"\n{Colors.WARNING}set quality to {quality}.{Colors.END}")
    asyncio.run(main(quality))
