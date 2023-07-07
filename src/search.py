import sqlite3

def search_cid_by_filename(filename, totalhash):
    conn = sqlite3.connect('credits.db')
    cursor = conn.cursor()

    # Replace 'totalhash' with the actual table name
    query = "SELECT cid FROM '{}' WHERE filename = ?".format(totalhash)
    cursor.execute(query, (filename,))
    result = cursor.fetchone()

    cursor.close()
    conn.close()

    if result:
        return result[0]  # Return the first column (CID) of the result
    else:
        return None

# Example usage
filename = "ComputerNetworks-Abstract.pdf"  # Replace with the desired filename
totalhash = "7307a5256ed77dfdd80c595a4fec92c7807df6a9"
cid = search_cid_by_filename(filename, totalhash)

if cid:
    print(f"CID for file '{filename}' found: {cid}")
else:
    print(f"CID not found for file '{filename}'")