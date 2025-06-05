import psycopg2

# Verbindung zur Datenbank herstellen
conn = psycopg2.connect(
    host="localhost",
    port=5432,
    database="to_do",
    user="postgres",
    password="password"
)

cur = conn.cursor()

# 1. Tabellen auflisten
cur.execute("""
    SELECT table_name
    FROM information_schema.tables
    WHERE table_schema = 'public'
""")
tables = cur.fetchall()

print("\n📦 Tabellen in der Datenbank:\n")
for (table_name,) in tables:
    print(f"🔹 {table_name}")

print("\n📐 Tabellenstruktur & Beispieldaten:\n")

for (table_name,) in tables:
    print(f"\n=== 🧾 Tabelle: {table_name} ===\n")

    # 2. Spaltenstruktur
    cur.execute(f"""
        SELECT column_name, data_type
        FROM information_schema.columns
        WHERE table_name = %s
    """, (table_name,))
    columns = cur.fetchall()
    for col, dtype in columns:
        print(f" - {col}: {dtype}")

    # 3. Erste 10 Datensätze anzeigen
    try:
        cur.execute(f"SELECT * FROM {table_name} LIMIT 10")
        rows = cur.fetchall()
        if rows:
            print("\n🔍 Beispiel-Daten:")
            for row in rows:
                print(row)
        else:
            print("⚠️  Keine Daten vorhanden.")
    except Exception as e:
        print(f"⚠️  Fehler beim Abrufen von Daten: {e}")

cur.close()
conn.close()
