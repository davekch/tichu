import pygame as pg
from pygame.color import THECOLORS as COLORS
from client import Client


WIDTH, HEIGHT = 800, 600
FRAMERATE = 30
pg.font.init()
FONT = pg.font.Font(None, 32)

C_BUTTON = COLORS["darkseagreen1"]
C_BUTTON_PRESSED = COLORS["darkseagreen4"]
C_TEXT = COLORS["gray14"]
C_TEXTBOX_ACTIVE = COLORS["darkturquoise"]
C_TEXTBOX_INACTIVE = COLORS["azure2"]


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

    def handle_event(self, event):
        if event.type == pg.MOUSEBUTTONDOWN:
            if self.rectangle.collidepoint(event.pos):
                self.pressed = True
                if callable(self.on_click):
                    return self.on_click()
        elif event.type == pg.MOUSEBUTTONUP:
            if self.pressed:
                self.pressed = False

    def draw(self, screen):
        color = C_BUTTON if not self.pressed else C_BUTTON_PRESSED
        pg.draw.rect(screen, color, self.rectangle, 0)
        text = FONT.render(self.text, True, C_TEXT)
        screen.blit(text, (self.rectangle.x + self.rectangle.w/2 - text.get_width()/2, self.rectangle.y + 10))


class TichuGui:
    def __init__(self):
        self.client = Client()
        self.running = True

        pg.init()
        self.screen = pg.display.set_mode((WIDTH, HEIGHT))
        pg.display.set_caption("Online-Tichu")
        pg.mouse.set_visible(1)
        self.clock = pg.time.Clock()

    def login_screen(self):
        logged_in = False
        username_box = TextInputBox(WIDTH/2 - 150, HEIGHT/2 - 75, 300, 40, "username")
        addr_box = TextInputBox(WIDTH/2 - 150, HEIGHT/2 - 20, 300, 40, "IP:port")
        go_button = Button(WIDTH/2 - 150, HEIGHT/2 + 30, 300, 40, "Go!", on_click=lambda: (username_box.text, addr_box.text))
        while not logged_in and self.running:
            self.clock.tick(FRAMERATE)
            self.screen.fill(COLORS["white"])
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
                self.client.connect(username, ip, int(port))
                logged_in = True

            pg.display.flip()

    def wait_screen(self):
        text = FONT.render("wait for the others to connect ...", True, C_TEXT)
        while self.running:
            self.clock.tick(FRAMERATE)
            self.screen.fill(COLORS["white"])
            self.screen.blit(text, (WIDTH/2 - text.get_width()/2, HEIGHT/2 - 20))
            pg.display.flip()


if __name__ == "__main__":
    tichu = TichuGui()
    tichu.login_screen()
    tichu.wait_screen()
