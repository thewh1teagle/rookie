from typing import List
from sys import platform


def firefox(domains: List[str] = None) -> List[dict]:
    """
    Extract Cookies from Firefox

    :param domains: Optional list of domains to extract only from them
    :return: A list of dictionaries of cookies
    """
    ...


def firefox_based(key_path: str, db_path: str, domains: List[str] = None) -> List[dict]:
    """
    Extract Cookies from Firefox-based browsers

    :param key_path: Path to the key file
    :param db_path: Path to the database file
    :param domains: Optional list of domains to extract only from them
    :return: A list of dictionaries of cookies
    """
    ...


def brave(domains: List[str] = None) -> List[dict]:
    """
    Extract Cookies from Brave browser

    :param domains: Optional list of domains to extract only from them
    :return: A list of dictionaries of cookies
    """
    ...


def edge(domains: List[str] = None) -> List[dict]:
    """
    Extract Cookies from Microsoft Edge browser

    :param domains: Optional list of domains to extract only from them
    :return: A list of dictionaries of cookies
    """
    ...


def chrome(domains: List[str] = None) -> List[dict]:
    """
    Extract Cookies from Google Chrome browser

    :param domains: Optional list of domains to extract only from them
    :return: A list of dictionaries of cookies
    """
    ...


def chromium_based(db_path: str, domains: List[str] = None) -> List[dict]:
    """
    Extract Cookies from Chromium-based browsers

    :param db_path: Path to the database file
    :param domains: Optional list of domains to extract only from them
    :return: A list of dictionaries of cookies
    """
    ...


def chromium(domains: List[str] = None) -> List[dict]:
    """
    Extract Cookies from Chromium browser

    :param domains: Optional list of domains to extract only from them
    :return: A list of dictionaries of cookies
    """
    ...


def opera(domains: List[str] = None) -> List[dict]:
    """
    Extract Cookies from Opera browser

    :param domains: Optional list of domains to extract only from them
    :return: A list of dictionaries of cookies
    """
    ...


def vivaldi(domains: List[str] = None) -> List[dict]:
    """
    Extract Cookies from Vivaldi browser

    :param domains: Optional list of domains to extract only from them
    :return: A list of dictionaries of cookies
    """
    ...


def opera_gx(domains: List[str] = None) -> List[dict]:
    """
    Extract Cookies from Opera GX browser

    :param domains: Optional list of domains to extract only from them
    :return: A list of dictionaries of cookies
    """
    ...


def libre_wolf(domains: List[str] = None) -> List[dict]:
    """
    Extract Cookies from LibreWolf browser

    :param domains: Optional list of domains to extract only from them
    :return: A list of dictionaries of cookies
    """
    ...


def load(domains: List[str] = None) -> List[dict]:
    """
    Load Cookies from a browser

    :param domains: Optional list of domains to load cookies from
    :return: A list of dictionaries of cookies
    """
    ...


def any_browser(domains: List[str] = None) -> List[dict]:
    """
    Extract Cookies from any browser

    :param domains: Optional list of domains to extract only from them
    :return: A list of dictionaries of cookies
    """
    ...


# Windows
if platform == "win32":
    def internet_explorer(domains: List[str] = None) -> List[dict]:
        """
        Extract Cookies from Internet Explorer

        :param domains: Optional list of domains to extract only from them
        :return: A list of dictionaries of cookies
        """
        ...

    def octo_browser(domains: List[str] = None) -> List[dict]:
        """
        Extract Cookies from Octo browser

        :param domains: Optional list of domains to extract only from them
        :return: A list of dictionaries of cookies
        """
        ...


# MacOS
if platform == "darwin":
    def safari(domains: List[str] = None) -> List[dict]:
        """
        Extract Cookies from Safari browser

        :param domains: Optional list of domains to extract only from them
        :return: A list of dictionaries of cookies
        """
        ...


