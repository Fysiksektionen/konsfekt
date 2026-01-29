import sys, urllib.request, json, os, random, sqlite3
from PIL import Image

try:
    COUNT = min(abs(int(sys.argv[1])), 70)
except:
    print("Usage: mockdata.py {count}")
    quit()

IMG_DISK_SIZE = 512
IMG_DISK_PATH = "./db/uploads/images/product/"

conn = sqlite3.connect("./db/db.sqlite")
cur = conn.cursor()

flags = "{\"modifiable\": true, \"new_product\": false, \"marked_sold_out\": false}"

with open("./scripts/dogs_metadata.json", "r") as file:
    dogs = [dog | {"flags": flags, "stock": random.randint(0, 100)} for dog in json.load(file)]

random.shuffle(dogs)

for dog in dogs[:COUNT]:
    cur.execute("INSERT INTO Product (name, price, description, stock, flags) VALUES (:name, :price, :description, :stock, :flags)", dog)

    id = cur.lastrowid

    content = urllib.request.urlopen("https://dog.ceo/api/breeds/image/random").read()
    url = json.loads(content)["message"]
    urllib.request.urlretrieve(url, "./scripts/temp_image")
    img = Image.open("./scripts/temp_image")
    w, h = img.size
    side = min(w, h)
    x = y = 0
    if w > h:
        x = (w-side)//2
    else:
        y = (h-side)//2
    img = img.crop((x, y, side, side))
    img = img.resize((IMG_DISK_SIZE, IMG_DISK_SIZE), Image.BICUBIC)
    img.save(f"{IMG_DISK_PATH}{id}.webp", "WEBP", lossless=True)

    conn.commit()

os.remove("./scripts/temp_image")