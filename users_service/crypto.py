import hashlib
import random
import string


def hash_password(password: str, salt: str) -> str:
    hash = hashlib.sha256()
    hash.update(password.encode('utf-8'))
    hash.update(salt.encode('utf-8'))
    return hash.hexdigest()


def generate_salt() -> str:
    return "".join(random.choices(string.ascii_letters + string.digits, k=16))
