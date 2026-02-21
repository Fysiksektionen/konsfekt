import urllib.request, json, os, random, sqlite3, argparse
import sys, time, itertools
from PIL import Image

parser = argparse.ArgumentParser(
    description="Generated mock data for the project"
)

parser.add_argument(
    "-t", "--type",
    choices=["product", "user", "transaction"],
    required=True,
    help="Type of data to be generated"
)

parser.add_argument(
    "count",
    type=int,
    help="Amount of mock data to be generated"
)

args = parser.parse_args()

DB_PATH = "./db/db.sqlite"

IMG_DISK_SIZE = 512
IMG_DISK_PATH = "./db/uploads/images/product/"
PRODUCT_METADATA_PATH = "./scripts/dogs_metadata.json"
PRODUCT_IMAGE_API_URL = "https://dog.ceo/api/breeds/image/random"
TEMP_IMAGE_PATH = "./scripts/temp_image"

USER_METADATA_PATH = "./scripts/user_metadata.json"

conn = sqlite3.connect("./db/db.sqlite")
cur = conn.cursor()

def add_products(conn, cur, count):
    if abs(count) > 70:
        parser.error("Cannot create more than 70 products at a time")

    flags = "{\"modifiable\": true, \"new_product\": false, \"marked_sold_out\": false}"

    with open(PRODUCT_METADATA_PATH, "r") as file:
        dogs = [dog | {"flags": flags, "stock": random.randint(0, 100)} for dog in json.load(file)]

    random.shuffle(dogs)

    for dog in dogs[:count]:
        cur.execute("INSERT INTO Product (name, price, description, stock, flags) VALUES (:name, :price, :description, :stock, :flags)", dog)

        id = cur.lastrowid

        content = urllib.request.urlopen(PRODUCT_IMAGE_API_URL).read()
        url = json.loads(content)["message"]
        urllib.request.urlretrieve(url, TEMP_IMAGE_PATH)
        img = Image.open(TEMP_IMAGE_PATH)
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
        yield

    os.remove(TEMP_IMAGE_PATH)


def add_users(conn, cur, count):
    with open(USER_METADATA_PATH, "r") as file:
        names = json.load(file)
        random.shuffle(names)
     
    for name in names[:count]:
        cur.execute("INSERT INTO User (name, email, google_id, role, balance, on_leaderboard, private_transactions) VALUES (:name, :email, :google_id, :role, :balance, :on_leaderboard, :private_transactions)", {
            "name": name,
            "email": name.lower() + "@fysiksektionen.se",
            "google_id": hash(name + str(time.time())),
            "role": "user",
            "balance": 0,
            "on_leaderboard": random.choice([0,1]),
            "private_transactions": random.choice([0,1])
        })
        conn.commit()
        yield

def add_transactions(conn, cur, count):
    cur.execute("SELECT id FROM User")
    user_ids = [row[0] for row in cur.fetchall()]
    if not user_ids:
        parser.error("No users found. Add users first.")

    cur.execute("SELECT id, name, price FROM Product")
    products = [{"id": row[0], "name": row[1], "price": row[2]} for row in cur.fetchall()]
    if not products:
        parser.error("No products found. Add products first.")

    now = time.time()

    for _ in range(count):
        user_id = random.choice(user_ids)
        datetime = int(now - random.uniform(0, 60 * 60 * 24 * 90))

        if random.random() < 0.2:
            amount = round(random.uniform(50, 500), 2)
            cur.execute(
                "INSERT INTO StoreTransaction (user, amount, datetime) VALUES (?, ?, ?)",
                (user_id, amount, datetime)
            )
        else:
            selected = random.sample(products, k=min(random.randint(1, 3), len(products)))
            quantities = {p["id"]: random.randint(1, 3) for p in selected}
            amount = -round(sum(p["price"] * quantities[p["id"]] for p in selected), 2)
            cur.execute(
                "INSERT INTO StoreTransaction (user, amount, datetime) VALUES (?, ?, ?)",
                (user_id, amount, datetime)
            )
            tx_id = cur.lastrowid
            for p in selected:
                cur.execute(
                    "INSERT INTO TransactionItem (transaction_id, product, quantity, name, price) VALUES (?, ?, ?, ?, ?)",
                    (tx_id, p["id"], quantities[p["id"]], p["name"], p["price"])
                )

        conn.commit()
        yield


spinner = itertools.cycle("|/-\\")
time_start = time.time()

for i, _ in enumerate({
    "product": add_products,
    "user": add_users,
    "transaction": add_transactions
}[args.type](conn, cur, args.count)):
    print(f"\r{next(spinner)}\tGenerating {args.type} data ({i+1}/{args.count})", end="", flush=True)

elapsed = (time.time() - time_start)
print(f"\n\033[1;32mDone!\033[0m finished in \033[1m{elapsed:.2f}s\033[0m")
cur.close()
