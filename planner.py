import os
import random

from s3_util import S3Util
from ses_util import SESUtil
from trello import Trello

API_KEY = os.environ['API_KEY']
TOKEN = os.environ['TOKEN']
URL_BASE = os.environ['URL_BASE']
DINNER_LIST_ID = os.environ['DINNER_LIST_ID']
BUCKET = os.environ['BUCKET']
KEY = os.environ['KEY']
MY_EMAIL = os.environ['MY_EMAIL']
GF_EMAIL = os.environ['GF_EMAIL']

s3 = S3Util(BUCKET, KEY)
trello = Trello(API_KEY, TOKEN, URL_BASE, DINNER_LIST_ID)
ses = SESUtil(MY_EMAIL, [ MY_EMAIL, GF_EMAIL ])

# 1) Download suggest counts from S3.
suggest_counts = s3.get_suggest_counts()

# 2) Get all meals from Trello.
meals = trello.get_meals()

# 3) Check that all meals are in the suggest_counts, adding them if they're not.
for meal in meals:
    meal_id = meal['id']
    if meal_id not in suggest_counts:
        suggest_counts[meal_id] = 0

# 4) Give each meal a random likeliness rating: count * random.
scored = []
for meal_id, count in suggest_counts.items():
    scored.append((meal_id, float(count) * random.random()))

# 5) Sort the scored list in ascending order. Take the top 2.
prioritized = sorted(scored, key=lambda x: x[1])[:2]
fst_mid = prioritized[0][0]
snd_mid = prioritized[1][0]

# 6) Increment those two scores in the map
suggest_counts[fst_mid] += 1
suggest_counts[snd_mid] += 1

# 7) Send an email.
meals_to_send = [meal for meal in meals if meal['id'] == fst_mid or meal['id'] == snd_mid]
ses.send_email(meals_to_send)

# 8) update the suggest_counts object in s3
s3.update_suggest_counts(suggest_counts)
