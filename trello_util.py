import re
import requests

class TrelloUtil:
    def __init__(self, key, token, url_base, dinner_list_id):
        self.url = f'{url_base}/lists/{dinner_list_id}/cards'
        self.auth = { 'key': key, 'token': token }

    def split_once(self, word, text):
        return re.compile(word, flags=re.IGNORECASE).split(text, 1)

    def parse_ingredients(self, desc) -> str:
        try:
            [ingredients, _] = self.split_once('directions', desc)
            [_, cleaned_ingredients] = self.split_once('ingredients', ingredients)
            return cleaned_ingredients.replace('#', '')
        except Exception as e:    
            return ('I couldn\'t parse the description 😕. Check the ingredients by clicking the link.')

    def _to_card(self, c):
        return {
            'id': c['id'],
            'name': c['name'],
            'ingredients': self.parse_ingredients(c['desc']),
            'url': c['url']
        }

    def get_meals(self):
        print('Fetching cards from Trello.')
        cards = requests.get(url=self.url,params=self.auth).json()
        return [self._to_card(card) for card in cards]
