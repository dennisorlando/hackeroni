package ui.nav

import androidx.compose.animation.core.tween
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.foundation.background
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material3.DrawerValue
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.ModalNavigationDrawer
import androidx.compose.material3.rememberDrawerState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Modifier
import androidx.navigation.NavType
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import androidx.navigation.navArgument
import kotlinx.coroutines.launch
import ui.dummy.DummyScreen
import ui.home.HomeScreen
import ui.settings.SettingsScreen

@Composable
fun Navigation() {
    val navController = rememberNavController()
    val backIcon = @Composable {
        IconButton(
            onClick = {
                // avoid navigating up if the root destination is already being shown
                if (navController.currentDestination?.route != Route.Home.name) {
                    navController.navigateUp()
                }
            }
        ) {
            Icon(
                imageVector = Icons.AutoMirrored.Filled.ArrowBack,
                contentDescription = null,
            )
        }
    }

    NavHost(
        navController = navController,
        startDestination = Route.Home.name,
        enterTransition = { fadeIn(animationSpec = tween(400)) },
        exitTransition = { fadeOut(animationSpec = tween(400)) },
        modifier = Modifier.background(MaterialTheme.colorScheme.background),
    ) {
        composable(Route.Home.name) {
            ScreenWithDrawer(
                onSettingsClick = { navController.navigate(Route.Settings.name) },
                onDummyClick = { navController.navigate(Route.Dummy.name + "/42") },
            ) {
                HomeScreen(it)
            }
        }

        composable(Route.Settings.name) {
            SettingsScreen(backIcon)
        }

        composable(
            route = Route.Dummy.name + "/{number}",
            arguments = listOf(
                navArgument("number") { type = NavType.IntType }
            )
        ) {
            DummyScreen(it.arguments?.getInt("number") ?: -1, backIcon)
        }
    }
}

@Composable
fun ScreenWithDrawer(
    onSettingsClick: () -> Unit,
    onDummyClick: () -> Unit,
    screen: @Composable (navigationIcon: @Composable () -> Unit) -> Unit
) {
    val drawerState = rememberDrawerState(initialValue = DrawerValue.Closed)
    val scope = rememberCoroutineScope()

    ModalNavigationDrawer(
        drawerState = drawerState,
        drawerContent = {
            DrawerContent(
                onSettingsClick = onSettingsClick,
                onDummyClick = onDummyClick,
                closeDrawer = {
                    scope.launch {
                        drawerState.close()
                    }
                }
            )
        },
    ) {
        screen {
            AppBarDrawerIcon(
                onDrawerClick = {
                    scope.launch {
                        drawerState.apply {
                            if (isClosed) open() else close()
                        }
                    }
                },
                isClosed = drawerState.isClosed,
            )
        }
    }
}
