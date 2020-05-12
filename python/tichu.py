import pygame as pg
from pygame.color import THECOLORS as COLORS
import threading
import os
from functools import wraps
from client import Client

import logging

logger = logging.getLogger("tichu")


WIDTH, HEIGHT = 1300, 800
FRAMERATE = 30
pg.font.init()
FONT = pg.font.Font(None, 32)
CARD_WIDTH = 60
CARD_HEIGHT = 90

C_BACKGROUND = COLORS["white"]
C_BUTTON = COLORS["darkseagreen1"]
C_BUTTON_DISABLED = COLORS["darkgray"]
C_BUTTON_PRESSED = COLORS["darkseagreen4"]
C_TEXT = COLORS["gray14"]
C_TEXTBOX_ACTIVE = COLORS["darkturquoise"]
C_TEXTBOX_INACTIVE = COLORS["azure2"]

PATH = os.path.dirname(__file__)
RESOURCES_PATH = os.path.join(PATH, "resources")
SYMBOL_MAP = {
    "two": "2",
    "three": "3",
    "four": "4",
    "five": "5",
    "six": "6",
    "seven": "7",
    "eight": "8",
    "nine": "9",
    "ten": "10",
    "jack": "J",
    "queen": "Q",
    "king": "K",
    "ace": "A",
}


class TextInputBox:
    # mostly copied from https://stackoverflow.com/questions/46390231/how-to-create-a-text-input-box-with-pygame
    def __init__(self, x, y, width, height, text=""):
        self.rectangle = pg.Rect(x, y, width, height)
        self.text = text
        self.active = False
        self.rendered = FONT.render(self.text, True, C_TEXT)

    def update(self, event):
        if event.type == pg.MOUSEBUTTONDOWN:
            # check if mouse clicked happened inside this rect
            if self.rectangle.collidepoint(event.pos):
                self.text = "" if not self.active else self.text
                self.active = True
            else:
                self.active = False
        elif event.type == pg.KEYDOWN:
            if self.active:
                if event.key == pg.K_RETURN:
                    self.active = False
                elif event.key == pg.K_BACKSPACE:
                    self.text = self.text[:-1]
                else:
                    self.text += event.unicode

        # Re-render the text.
        self.rendered = FONT.render(self.text, True, C_TEXT)

    def draw(self, screen):
        pg.draw.rect(screen, C_TEXTBOX_INACTIVE, self.rectangle, 0)
        if self.active:
            # draw border
            pg.draw.rect(screen, C_TEXTBOX_ACTIVE, self.rectangle, 2)
        screen.blit(self.rendered, (self.rectangle.x + 5, self.rectangle.y + 10))


class Button:
    def __init__(self, x, y, width, height, text="", on_click=None):
        self.rectangle = pg.Rect(x, y, width, height)
        self.text = text
        self.on_click = on_click
        self.pressed = False
        self.enabled = True

    def handle_event(self, event):
        if event.type == pg.MOUSEBUTTONDOWN:
            if self.rectangle.collidepoint(event.pos):
                self.pressed = True
        elif event.type == pg.MOUSEBUTTONUP:
            if self.pressed:
                self.pressed = False
                # call on_click on release of the button
                if callable(self.on_click):
                    return self.on_click()

    def draw(self, screen):
        if not self.enabled:
            color = C_BUTTON_DISABLED
        elif self.pressed:
            color = C_BUTTON_PRESSED
        else:
            color = C_BUTTON

        pg.draw.rect(screen, color, self.rectangle, 0)
        text = FONT.render(self.text, True, C_TEXT)
        screen.blit(
            text,
            (
                self.rectangle.x + self.rectangle.w / 2 - text.get_width() / 2,
                self.rectangle.y + 10,
            ),
        )


class Card(pg.Rect):
    def __init__(self, x, y, name):
        # save original coordinates as the current coordinates may change via drag and drop
        self.x0 = x
        self.y0 = y
        pg.Rect.__init__(self, x, y, CARD_WIDTH, CARD_HEIGHT)
        # special cards don't have a space in their name
        if " " in name:
            self.color, value = name.split()
            self.symbol = pg.image.load(
                os.path.join(RESOURCES_PATH, self.color + ".png")
            )
            self.text = SYMBOL_MAP[value.lower()]
        else:
            self.symbol = pg.image.load(os.path.join(RESOURCES_PATH, name + ".png"))
        # is this card being dragged around right now
        self.dragged = False

    def draw(self, screen):
        pg.draw.rect(screen, C_TEXT, self, 2)  # draw border of rectangle
        screen.blit(self.symbol, (self.x - 20, self.y + 5))
        if hasattr(self, "text"):
            screen.blit(
                FONT.render(self.text, True, COLORS[self.color]),
                (self.x + CARD_WIDTH - 25, self.y + 5),
            )


class Hand(pg.Rect):
    def __init__(self, x, y, width, height):
        pg.Rect.__init__(self, x, y, width, height)
        self.cardbuttons = []

    def set_cards(self, cardnames):
        self.cardbuttons = []
        if len(cardnames) == 0:
            return

        # calculate the space the cards will need
        space = 20  # space between 2 cards
        needed_width = CARD_WIDTH * len(cardnames) + space * (len(cardnames) - 1)
        # x coordinate of first card
        x0 = self.x + int(self.width / 2) - int(needed_width / 2)
        y0 = self.y + 20
        for i, card in enumerate(cardnames):
            x = x0 + i * (CARD_WIDTH + space)
            self.cardbuttons.append(Card(x, y0, card))

    def draw(self, screen):
        pg.draw.rect(screen, C_TEXT, self, 3)
        for card in self.cardbuttons:
            card.draw(screen)


class CardArea:
    """displays the player's cards and stage + handles drag & drop of cards
    """

    def __init__(self, x, y, width, height, callbackobject):
        self.hand = Hand(x, y, width, height / 2 - 10)
        self.stage = Hand(x, y - height / 2 - 20, width, height / 2 - 10)
        # this will contain a triple of the card being dragged, its index and either "hand" or "stage"
        # depending on where the card is from
        self.dragged_card = None
        self.callbackmatrix = {
            "hand": {"hand": callbackobject.move_hand, "stage": callbackobject.stage},
            "stage": {"hand": callbackobject.unstage, "stage": callbackobject.move_stage},
        }
        self.callbackobject = callbackobject

    def set_hand(self, cardnames):
        self.hand.set_cards(cardnames)

    def set_stage(self, cardnames):
        self.stage.set_cards(cardnames)

    def draw(self, screen):
        self.hand.draw(screen)
        self.stage.draw(screen)

    def handle_event(self, event):
        pos = pg.mouse.get_pos()
        # first, move around a card if one is being dragged
        if self.dragged_card:
            card, _, _ = self.dragged_card
            card.x, card.y = pos

        if event.type == pg.MOUSEBUTTONDOWN:
            # check if we hit a card and if yes, from which stack it comes
            cardstack = None
            # remember from where we're about to drag a card
            if self.hand.collidepoint(pos):
                cardstack = self.hand.cardbuttons
                stackname = "hand"
            elif self.stage.collidepoint(pos):
                cardstack = self.stage.cardbuttons
                stackname = "stage"
            if cardstack is not None:
                # check for collisions with cards
                for i, card in enumerate(cardstack):
                    if card.collidepoint(pos):
                        self.dragged_card = (card, i, stackname)
                        logger.debug("picked card {} from {}".format(i, stackname))
                        break

        elif event.type == pg.MOUSEBUTTONUP:
            if not self.dragged_card:
                # nothing to do ...
                return
            # is a card dropped into emptiness?
            if not (self.hand.collidepoint(pos) or self.stage.collidepoint(pos)):
                card, _, _ = self.dragged_card
                card.x, card.y = card.x0, card.y0
                self.dragged_card = None
                return

            if self.hand.collidepoint(pos):
                targetstack = self.hand.cardbuttons
                targetname = "hand"
            elif self.stage.collidepoint(pos):
                targetstack = self.stage.cardbuttons
                targetname = "stage"

            logger.debug("going to drop onto {}".format(targetname))
            card, i, sourcename = self.dragged_card
            # calculate targetindex j
            if len(targetstack) == 0:
                logger.debug("aptly {} has len 0".format(targetname))
                j = 0
            elif pos[0] > targetstack[-1].x:
                logger.debug("mouselocation {} is larger than {}, drop at end".format(pos[0], targetstack[0].x))
                j = len(targetstack) - 1
            else:
                for k, c in enumerate(targetstack):
                    logger.debug("comparing mousex {} with card {} at pos {}".format(pos[0], k, c.x))
                    if c.collidepoint(pos) and not i == k:
                        j = k
                        break
                    elif pos[0] < c.x:
                        j = max(k - 1, 0)
                        break
            # call the callback
            self.callbackmatrix[sourcename][targetname](i, j)
            # update hand and stage
            self.set_hand(self.callbackobject._hand)
            self.set_stage(self.callbackobject._stage)
            self.dragged_card = None


class TichuGui:
    def __init__(self):
        self.client = Client()
        self.running = True
        # this is true if all others are connected and the game is running
        self.on_main = False
        self.threads = []

        pg.init()
        self.screen = pg.display.set_mode((WIDTH, HEIGHT))
        pg.display.set_caption("Online-Tichu")
        pg.mouse.set_visible(1)
        self.clock = pg.time.Clock()

    def login_screen(self):
        logged_in = False
        username_box = TextInputBox(
            WIDTH / 2 - 150, HEIGHT / 2 - 75, 300, 40, "username"
        )
        addr_box = TextInputBox(
            WIDTH / 2 - 150, HEIGHT / 2 - 20, 300, 40, "127.0.0.1:1001"
        )
        go_button = Button(
            WIDTH / 2 - 150,
            HEIGHT / 2 + 30,
            300,
            40,
            "Go!",
            on_click=lambda: (username_box.text, addr_box.text),
        )
        while not logged_in and self.running:
            self.clock.tick(FRAMERATE)
            self.screen.fill(C_BACKGROUND)
            for event in pg.event.get():
                if event.type == pg.QUIT:
                    self.running = False
                else:
                    username_box.update(event)
                    addr_box.update(event)
                    result = go_button.handle_event(event)

            username_box.draw(self.screen)
            addr_box.draw(self.screen)
            go_button.draw(self.screen)

            if result:
                username, addr = result
                ip, port = addr.split(":")

                # helper function for new thread
                def connect():
                    self.client.connect(username, ip, int(port))
                    self.on_main = True

                _t = threading.Thread(target=connect, daemon=True)
                _t.start()
                self.threads.append(_t)
                logged_in = True

            pg.display.flip()

    def wait_screen(self):
        text = FONT.render("wait for the others to connect ...", True, C_TEXT)
        while self.running and not self.on_main:
            # self.on_main gets set to true as soon as the thread started in login_screen
            # is finished
            self.clock.tick(FRAMERATE)
            for event in pg.event.get():
                if event.type == pg.QUIT:
                    self.running = False

            self.screen.fill(C_BACKGROUND)
            self.screen.blit(text, (WIDTH / 2 - text.get_width() / 2, HEIGHT / 2 - 20))
            pg.display.flip()

        if self.running:
            # join the connect-thread (it is now finished)
            self.threads.pop().join()

    def main_screen(self):

        card_area = CardArea(
            50,
            HEIGHT - CARD_HEIGHT - 80,
            WIDTH - 100, 2 * (CARD_HEIGHT + 40) + 20,
            callbackobject=self.client
        )
        # callback function for take_hand_button
        def take_hand():
            self.client.request_cards()
            card_area.set_hand(self.client._hand)

        # TODO: on_click: disable this button + error handling
        take_hand_button = Button(50, 50, 180, 40, "take new cards", on_click=take_hand)

        while self.running:
            self.clock.tick(FRAMERATE)
            for event in pg.event.get():
                if event.type == pg.QUIT:
                    self.running = False
                else:
                    take_hand_button.handle_event(event)
                    card_area.handle_event(event)
                    # hand_cards.handle_event(
                    #     event,
                    #     callback=lambda i, j: (
                    #         self.client.move_hand(i, j),
                    #         hand_cards.set_cards(self.client._hand),
                    #     ),
                    # )

            self.screen.fill(C_BACKGROUND)
            take_hand_button.draw(self.screen)
            card_area.draw(self.screen)

            pg.display.flip()

    def quit(self):
        logger.info("quitting pygame ... ")
        self.client.disconnect()
        pg.display.quit()
        pg.quit()


if __name__ == "__main__":
    logging.basicConfig(
        level=logging.DEBUG,
        format="%(asctime)s [%(levelname)-8s] %(name)s.%(funcName)s: %(message)s",
        datefmt="%H:%M:%S",
    )

    tichu = TichuGui()
    tichu.login_screen()
    tichu.wait_screen()
    tichu.main_screen()
    tichu.quit()
