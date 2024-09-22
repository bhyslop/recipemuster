
from IPython.display import Markdown, display
import anthropic

from collections import Counter

def sort_words_by_first_letter_frequency(words):
    # Define the order of letters from least frequent to most frequent
    letter_order = "XZQJKVYWFBGHMLNREDUIOPCTAS"
    
    # Create a dictionary to map each letter to its index in the order
    letter_rank = {letter: index for index, letter in enumerate(letter_order)}
    
    def get_sort_key(word):
        # Convert the first letter of the word to uppercase
        first_letter = word[0].upper()
        # Return the rank of the first letter, or the highest rank + 1 if not found
        return letter_rank.get(first_letter, len(letter_order))
    
    # Sort the words using the custom key function
    return sorted(words, key=get_sort_key)


def filter_and_sort_names(names, min_letters, max_letters):
    # Filter names based on length
    valid_names = [name for name in names if min_letters <= len(name) <= max_letters]
    rejected_names = [name for name in names if name not in valid_names]

    # Calculate frequency of initial letters
    initial_letters = [name[0].lower() for name in valid_names]
    letter_frequency = Counter(initial_letters)

    sorted_valid_names = sort_words_by_first_letter_frequency(valid_names)

    return sorted_valid_names, rejected_names


def as_markdown_table(word_list):
    table  = "| Word 1 | Word 2 | Word 3 | Word 4 | Word 5 |\n"
    table += "|--------|--------|--------|--------|--------|\n"

    for i in range(0, len(content), 5):
        row = content[i:i+5]
        row += [''] * (5 - len(row))  # Pad with empty strings if less than 5 words
        table += f"| {' | '.join(row)} |\n"
    return table


def name_help(hint, min_letters=2, max_letters=8, target_count=100):

    composite_hint = hint + f".  Valid words have at least {min_letters} and at most {max_letters} letters.  Try to provide {target_count} response words. "

    client = anthropic.Anthropic()
    message = client.messages.create(
        model="claude-3-haiku-20240307",
        # model="claude-3-5-sonnet-20240620",
        max_tokens=1000,
        temperature=0,
        system="""
        You are a brilliant and witty selector of words with really good gestalts.
        Terse, offering no explanations, you provide a list of interesting words
        that match the theme. 
        """,
        messages=[
            {"role": "user", "content": composite_hint },
            {"role": "assistant", "content": "Your words are:"}  # Prefill
        ]
    )
    content = message.content[0].text.strip().split('\n')

    
    valid_words, rejected_words = filter_and_sort_names(content, min_letters, max_letters)

    # OUCH: ??? parameterize above func for count and letter count range, then
    #                   order below accordingly, maybe split to 2 tables.
    # OUCH: ??? sort by rarer words in english language first. 
    # OUCH: ??? Work out means of putting the python in a .py to hide it
    # Create markdown table
    mkd = []
    mkd += ['## Valid words']
    mkd +=  as_markdown(valid_words)
    mkd += ['## Reject words']
    mkd +=  as_markdown(rejected_words)
    mkd += ['## Prompt echoback']
    mkd += [composite_hint]
    
    display(Markdown(table))