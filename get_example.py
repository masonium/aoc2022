#!/home/mason/venvs/emacs3env/bin/python3
import bs4
import requests
from typing import List, Tuple
import datetime
import click

def get_aoc_prompt(year: int, day: int) -> str:
    """Get the AOC prompt from a particular year and day."""
    session = ""
    with open("session.txt") as f:
        for line in f:
            session = line.strip()
            break

    resp = requests.request(url=f"https://adventofcode.com/{year}/day/{day}", method="GET", cookies={"session": session})
    return resp.text


def extract_example(prompt: str, index: int = 0) -> Tuple[str, List[int]]:
    """Extract the example from the prompt.

    By default, the example is assumed to be in the 'code' block. But
    it you can also specifiy the nth codeblock.
    """

    soup = bs4.BeautifulSoup(prompt, 'html.parser')

    pres = soup.find_all("pre")

    all_examples = []
    for pre in pres:
        for code in pre.find_all("code"):
            all_examples.append(code.contents[0])

    all_example_ans: List[int] = []
    for code in soup.find_all("code"):
        for em in code.find_all("em"):
            try:
                all_example_ans.append(em.contents[0])
            except:
                pass

    return all_examples[index], all_example_ans

@click.command()
@click.option("--day", default=None, type=int, help="day number to grab example from")
@click.option("--year", default=None, type=int, help="competition year to grab example from")
@click.option("--example-output", default=None, help="output file to write example contents to")
@click.option("--example-answer-output", default=None, help="output file to write example answer to")
def fetch_example(day: int, year: int, example_output: str, example_answer_output: str):
    """Fetch the example from the proper day."""
    now = datetime.datetime.now()
    today = now.replace(tzinfo=datetime.timezone(datetime.timedelta(hours=-11))).astimezone(tz=None)
    year = year or today.year
    day = day or today.day

    print(f"Getting example(s) for day {day}")
    prompt = get_aoc_prompt(year, day)

    example_output_path = example_output or f"input/example{day:02d}.in"
    example_answer_output_path = example_answer_output or f"input/example{day:02d}_answer.in"
    example, example_ans = extract_example(prompt)

    with open(example_output_path, "wt") as f:
        f.write(example)
    with open(example_answer_output_path, "wt") as f:
        for ans in example_ans:
            print(ans, file=f)


if __name__ == "__main__":
    fetch_example()
